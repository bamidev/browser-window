//! This module contains runtime and application related handles.
//! 
//! _Browser Window_ needs to be initialized, and also run its own runtime.
//! Once that is set up and running, all windows can be constructed and be used.
//! To do this, use `Application::initialize`.
//! Then you will have an `Application` instance, from which you can derive a `Runtime` instance.
//! Running the `Runtime` will grant you access to an `ApplicationHandle` which you use to manipulate your application with.
//!
//! # Example #1
//! Here is an example to show how you can construct your application:
//! ```
//! use std::process;
//! use browser_window::application::*;
//!
//! fn main() {
//! 	let application = Application::initialize( &ApplicationSettings::default() ).unwrap();
//! 	let runtime = application.start();
//!
//! 	let exit_code = runtime.run_async(|handle| async move {
//!
//! 		// Do something ...
//! 
//! 		// Not normally needed:
//! 		handle.exit(0);
//! 	});
//! 
//! 	process::exit(exit_code);
//! }
//! ```
//!
#![cfg_attr(not(feature = "threadsafe"), doc = r#"
_Browser Window_ also supports manipulating the GUI from other threads with thread-safe handles.
To use these, enable the `threadsafe` feature.
"#)]
#![cfg_attr(feature = "threadsafe", doc = r#"
# Example #2

If you want to run another kind of runtime, like [tokio](https://tokio.rs/) for example, its still possible to use _Browser Window_ in conjunction with that.
However, you will need to enable feature `threadsafe`, as it will enable all threadsafe handles.
Here is an example:

```rust
use std::process;
use browser_window::application::*;
use tokio;

async fn async_main( app: ApplicationHandleThreaded ) {
	// Do something ...

	app.exit(0);
}

fn main() {
	let application = Application::initialize( &ApplicationSettings::default() ).unwrap();
	let bw_runtime = application.start();

	let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

	// First run our own runtime on the main thread
	let exit_code = bw_runtime.run(|_app| {
		let app = _app.into_threaded();

		// Spawn the main logic into the tokio runtime
		tokio_runtime.spawn( async_main( app ) );
	});

	process::exit(exit_code);
}
```"#)]



use std::env;
use std::ffi::{CString};
use std::future::Future;
use std::os::raw::{c_int};
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

use browser_window_core::application::*;
use lazy_static::lazy_static;

pub use browser_window_core::application::ApplicationSettings;

use crate::cookie::CookieJar;
#[cfg(feature = "threadsafe")]
use crate::delegate::*;
use crate::error;


/// Use this to initialize and start your application with.
pub struct Application {
	pub(in super) handle: ApplicationHandle
}

/// *Note:* Only available with feature `threadsafe` enabled.
///
/// A thread-safe application handle.
/// This handle also allows you to dispatch code to be executed on the GUI thread from any other thread.
#[cfg(feature = "threadsafe")]
#[derive(Clone, Copy)]
pub struct ApplicationHandleThreaded {
	pub(in super) handle: ApplicationHandle
}
#[cfg(feature = "threadsafe")]
unsafe impl Send for ApplicationHandleThreaded {}
#[cfg(feature = "threadsafe")]
unsafe impl Sync for ApplicationHandleThreaded {}

#[derive(Clone, Copy)]
/// A thread-unsafe application handle.
/// Often provided by for Browser Window.
pub struct ApplicationHandle {
	pub(in super) inner: ApplicationImpl
}

#[cfg(feature = "threadsafe")]
struct ApplicationDispatchData<'a> {

	handle: ApplicationHandle,
	func: Box<dyn FnOnce(ApplicationHandle) + Send + 'a>
}

// The trait to be implemented by all (user-level) handles that are able to return an ApplicationHandle.
// Like: Application, ApplicationAsync, BrowserWindow, BrowserWindowAsync
pub trait HasAppHandle {
	fn app_handle( &self ) -> ApplicationHandle;
}

/// The runtime to run the application with.
pub struct Runtime {
	pub(in super) handle: ApplicationHandle
}

/// The data that is available to a waker, allowing it to poll a future.
struct WakerData<'a> {
	handle: ApplicationHandle,
	future: Pin<Box<dyn Future<Output=()> + 'a>>
}



/// The future that dispatches a closure onto the GUI thread
#[cfg(feature = "threadsafe")]
pub type ApplicationDelegateFuture<'a,R> = DelegateFuture<'a, ApplicationHandle, R>;



lazy_static! {
	static ref WAKER_VTABLE: RawWakerVTable = {
		RawWakerVTable::new(
			waker_clone,
			waker_wake,
			waker_wake_by_ref,
			waker_drop
		)
	};
}



impl Application {

	/// Prepares the os args as a vector of C compatible pointers.
	fn args_ptr_vec() -> (Vec<CString>, Vec<*mut u8>) {
		let args = env::args_os();
		let mut vec = Vec::with_capacity( args.len() );
		let mut vec_ptrs = Vec::with_capacity( args.len() );

		for arg in args {
			let string = CString::new( arg.to_string_lossy().to_string() ).expect("Unable to convert OsString into CString!");

			vec_ptrs.push( string.as_ptr() as _ );

			vec.push(
				string
			);
		}

		( vec, vec_ptrs )
	}

	/// Shuts down other processes and performs any necessary clean-up code.
	/// This is useful if you main function doesn't exit naturally.
	/// If you call `std::process::exit`, the variables currently available don't get dropped.
	/// This is problematic because Browser Window needs to shut down properly.
	/// Call this if you are using `exit` or doing something else to kill the process.
	pub fn finish( self ) {}

	/// In order to use the Browser Window API, you need to initialize Browser Window at the very start of your application.
	/// Preferably on the first line of your `main` function.
	///
	/// # Warning
	/// This will open another process of your application.
	/// Therefore, any code that will be placed before initialization will also be executed on all other processes.
	/// This is generally unnecessary.
	/// 
	/// # Arguments
	/// `settings` - Some settings that allow you to tweak some application behaviors.
	///              Use `Settings::default()` for default settings that work for most people.
	pub fn initialize( settings: &ApplicationSettings ) -> error::Result<Application> {

		let (args_vec, mut ptrs_vec) = Self::args_ptr_vec();
		let argc: c_int = args_vec.len() as _;
		let argv = ptrs_vec.as_mut_ptr();

		let core_handle = ApplicationImpl::initialize( argc, argv as _, settings )?;

		Ok(Application::from_core_handle( core_handle ))
	}

	/// Creates a `Runtime` from which you can run the application.
	pub fn start( &self ) -> Runtime {

		Runtime {
			handle: self.handle
		}
	}
}

impl Drop for Application {
	fn drop( &mut self ) {
		self.handle.inner.finish()
	}
}

impl Runtime {

	/// Polls a future given a pointer to the waker data.
	unsafe fn poll_future( data: *mut WakerData ) {
		debug_assert!( data != ptr::null_mut(), "WakerData pointer can't be zero!" );

		// TODO: Test if polling from the right thread

		let waker = Self::new_waker( data );
		let mut ctx = Context::from_waker( &waker );

		let result = (*data).future.as_mut().poll( &mut ctx );

		// When the future is ready, free the memory allocated for the waker data
		match result {
			Poll::Ready(_) => {
				Box::from_raw( data );
			},
			Poll::Pending => {}
		}
	}

	/// Constructs a `Waker` for our runtime
	unsafe fn new_waker( data: *mut WakerData ) -> Waker {
		debug_assert!( data != ptr::null_mut(), "WakerData pointer can't be zero!" );

		Waker::from_raw(
			RawWaker::new( data as _, &WAKER_VTABLE )
		)
	}

	/// Run the main loop and executes the given closure on it.
	///
	/// # Arguments
	/// * `on_ready` - This closure will be called when the runtime has initialized, and will provide the caller with an application handle.
	///
	/// # Reserved Codes
	/// -1 is used as the return code for when the main thread panicked during a delegated closure.
	pub fn run<H>( &self, on_ready: H ) -> i32 where
		H: FnOnce( ApplicationHandle )
	{
		return self._run( |handle| {
			on_ready( handle )
		} )
	}

	/// Runs the main loop and executes the given future within that loop.
	/// This function exits when the future finishes or when `exit` is called.
	///
	/// Keep in mind that calls to async functions or futures may not necessarily finish.
	/// Exiting the application causes the runtime to stop, and it doesn't necessarily complete all waiting tasks.
	///
	/// # Reserved Codes
	/// The same reserved codes apply as `run`.
	pub fn run_async<'a,C,F>( &'a self, func: C ) -> i32 where
		C: FnOnce( ApplicationHandle ) -> F + 'a,
		F: Future<Output=()> + 'a
	{
		self._run(|handle| {

			self.spawn( async move {
				func( handle.into() ).await;
			} );
		})
	}

	/// Use `run_async` instead.
	pub fn spawn<'a,F>( &'a self, future: F ) where
		F: Future<Output=()> + 'a
	{
		// Data for the waker.
		let waker_data = Box::into_raw( Box::new(
			WakerData {
				handle: self.handle.clone(),
				future: Box::pin( future )
			}
		) );

		// First poll
		unsafe { Runtime::poll_future( waker_data ) };
	}

	fn _run<'a,H>( &self, on_ready: H ) -> i32 where
		H: FnOnce( ApplicationHandle ) + 'a
	{
		let ready_data = Box::into_raw( Box::new( on_ready ) );

		self.handle.inner.run( ready_handler::<H>, ready_data as _ )
	}
}



impl Application {

	/// Constructs an `Application` from a ffi handle
	pub(in super) fn from_core_handle( inner: ApplicationImpl ) -> Self {
		Self {
			handle: ApplicationHandle::new( inner )
		}
	}
}

impl From<ApplicationHandle> for Application {
	fn from( other: ApplicationHandle ) -> Self {
		Self {
			handle: other
		}
	}
}



impl ApplicationHandle {

	pub fn cookie_jar(&self) -> CookieJar {
		CookieJar::global()
	}

	/// Causes the `Runtime` to terminate.
	/// The `Runtime`'s [`Runtime::run`] or spawn command will return the exit code provided.
	/// This will mean that not all tasks might complete.
	/// If you were awaiting
	pub fn exit( &self, exit_code: i32 ) {
		self.inner.exit( exit_code as _ );
	}

	pub(in super) fn new( inner: ApplicationImpl ) -> Self {
		Self {
			inner: inner
		}
	}

	/// **Note:** Only available with feature `threadsafe` enabled.
	///
	/// Transforms this application handle into a thread-safe version of it.
	#[cfg(feature = "threadsafe")]
	pub fn into_threaded( self ) -> ApplicationHandleThreaded {
		self.into()
	}

	/// Spawns the given future, executing it on the GUI thread somewhere in the near future.
	pub fn spawn<F>( &self, future: F ) where
		F: Future<Output=()> + 'static
	{
		// Data for the waker.
		let waker_data = Box::into_raw( Box::new(
			WakerData {
				handle: self.clone(),
				future: Box::pin( future )
			}
		) );

		// First poll
		unsafe { Runtime::poll_future( waker_data ) };
	}
}



#[cfg(feature = "threadsafe")]
impl ApplicationHandleThreaded {

	/// Executes the given closure `func` on the GUI thread, and gives back the result when done.
	/// This only works when the runtime is still running.
	/// If the closure panicked, or the runtime is not running, this will return an error.
	///
	/// The function signature is practically the same as:
	/// ```ignore
	/// pub async fn delegate<'a,F,R>( &self, func: F ) -> Result<R, DelegateError> where
	/// 	F: FnOnce( ApplicationHandle ) -> R + Send + 'a,
	/// 	R: Send { /* ... */ }
	/// ```
	///
	/// Keep in mind that in multi-threaded environments, it is generally a good idea to put the output on the heap.
	/// The output value _will_ be copied.
	///
	/// # Example
	/// ```ignore
	/// let my_value: String = app.delegate(|handle| {
	/// 	"string".to_owned()
	/// }).unwrap();
	/// ```
	pub fn delegate<'a,F,R>( &self, func: F ) -> ApplicationDelegateFuture<'a,R> where
		F: FnOnce( ApplicationHandle ) -> R + Send + 'a,
		R: Send
	{
		ApplicationDelegateFuture::<'a,R>::new( self.handle.clone(), |handle| {
			func( handle.into() )
		} )
	}

	/// Executes the given `future` on the GUI thread, and gives back its output when done.
	/// This only works when the runtime is still running.
	/// If the future panicked during a poll, or the runtime is not running, this will return an error.
	/// See also `delegate`.
	///
	/// The function signature is practically the same as:
	/// ```ignore
	/// pub async fn delegate_future<'a,F,R>( &self, func: F ) -> Result<R, DelegateError> where
	/// 	F: Future<Output=R> + 'static,
	/// 	R: Send { /* ... */ }
	/// ```
	///
	/// # Example
	/// ```ignore
	/// let my_value: String = app.delegate_future(async {
	/// 	"string".to_owned()
	/// }).unwrap();
	/// ```
	pub fn delegate_future<F,R>( &self, future: F ) -> DelegateFutureFuture<R> where
		F: Future<Output=R> + 'static,
		R: Send + 'static
	{
		DelegateFutureFuture::new( self.handle.clone(), future )
	}

	/// Executes the given async closure `func` on the GUI thread, and gives back the result when done.
	/// This only works when the runtime is still running.
	/// If the closure panicked, or the runtime is not running, this will return an error.
	///
	/// Except, async closures are not yet supported in stable Rust.
	/// What we actually mean are closures of the form:
	/// ```ignore
	/// |handle| async move { /* ... */ }
	/// ```
	///
	/// The function signature is practically the same as:
	/// ```ignore
	/// pub async fn delegate_async<'a,C,F,R>( &self, func: C ) -> Result<R, DelegateError> where
	/// 	C: FnOnce( ApplicationHandle ) -> F + Send + 'a,
	/// 	F: Future<Output=R>,
	/// 	R: Send + 'static
	/// { /* ... */ }
	/// ```
	///
	/// # Example
	/// ```ignore
	/// let my_value: String = app.delegate_async(|handle| async move {
	/// 	"String".to_owned()
	/// }).unwrap();
	/// ```
	pub fn delegate_async<'a,C,F,R>( &self, func: C ) -> DelegateFutureFuture<'a,R> where
		C: FnOnce( ApplicationHandle ) -> F + Send + 'a,
		F: Future<Output=R>,
		R: Send + 'static
	{
		let handle = self.handle.clone();
		DelegateFutureFuture::new( self.handle.clone(),async move {
			func( handle.into() ).await
		})
	}

	/// Queues the given closure `func` to be executed on the GUI thread somewhere in the future.
	/// The closure will only execute when and if the runtime is still running.
	/// Returns whether or not the closure will be able to execute.
	pub fn dispatch<'a,F>( &self, func: F ) -> bool where
		F:  FnOnce( ApplicationHandle ) + Send + 'a
	{
		let data_ptr = Box::into_raw( Box::new( ApplicationDispatchData {
			handle: self.handle,
			func: Box::new( func )
		} ) );

		self.handle.inner.dispatch( dispatch_handler, data_ptr as _ )
	}

	/// Queues the given async closure `func` to be executed on the GUI thread somewhere in the future.
	/// The closure will only execute when and if the runtime is still running.
	/// However, there is no guarantee that the whole closure will execute.
	/// The runtime might exit when the given closure is at a point of waiting.
	/// Returns whether or not the closure will be able to execute its first part.
	pub fn dispatch_async<'a,C,F>( &self, func: C ) -> bool where
		C: FnOnce( ApplicationHandle ) -> F + Send + 'a,
		F: Future<Output=()> + 'static
	{
		self.dispatch(|handle| {
			let future = func( handle );
			handle.spawn( future );
		})
	}

	/// Signals the runtime to exit.
	/// This will cause `Runtime::run` to stop and return the provided exit code.
	pub fn exit( &self, exit_code: i32 ) {
		// The thread-safe version of bw_Application_exit:
		self.handle.inner.exit_threadsafe( exit_code as _ );
	}

	/// Constructs an `ApplicationThreaded` handle from a ffi handle
	pub(in super) fn from_core_handle( inner: ApplicationImpl ) -> Self {
		Self {
			handle: ApplicationHandle::new( inner )
		}
	}

	/// Executes the given future on the GUI thread somewhere in the near future.
	pub fn spawn<F>( &self, future: F ) where
		F: Future<Output=()> + 'static
	{
		self.handle.spawn( future );
	}
}

#[cfg(feature = "threadsafe")]
impl From<ApplicationHandle> for ApplicationHandleThreaded {
	fn from( other: ApplicationHandle ) -> Self {
		Self {
			handle: other.clone()
		}
	}
}

#[cfg(feature = "threadsafe")]
impl Deref for ApplicationHandleThreaded {
	type Target = ApplicationHandle;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}



impl HasAppHandle for ApplicationHandle {

	fn app_handle( &self ) -> ApplicationHandle {
		self.clone()
	}
}



#[cfg(feature = "threadsafe")]
unsafe fn dispatch_handler( _app: ApplicationImpl, _data: *mut () ) {

	let data_ptr = _data as *mut ApplicationDispatchData<'static>;
	let data = Box::from_raw( data_ptr );

	(data.func)( data.handle.into() );
}

/// The handler that is invoked when the runtime is deemed 'ready'.
unsafe fn ready_handler<H>( handle: ApplicationImpl, user_data: *mut () ) where
	H: FnOnce( ApplicationHandle )
{
	let app = ApplicationHandle::new( handle );
	let closure = Box::from_raw( user_data as *mut H );

	closure( app );
}

/// A handler that is invoked by wakers.
unsafe fn wakeup_handler( _app: ApplicationImpl, user_data: *mut () ) {

	let	data = user_data as *mut WakerData;

	Runtime::poll_future( data );
}

unsafe fn waker_clone( data: *const () ) -> RawWaker {
	RawWaker::new( data, &WAKER_VTABLE )
}

unsafe fn waker_wake( data: *const () ) {
	let data_ptr = data as *const WakerData;

	(*data_ptr).handle.inner.dispatch( wakeup_handler, data_ptr as _ );
}

unsafe fn waker_wake_by_ref( data: *const () ) {
	waker_wake( data );
}

fn waker_drop( _data: *const () ) {}
