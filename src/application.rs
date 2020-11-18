//! This module contains runtime and application related handles.
//!
//! Browser Window needs to be initialized, and also run its own runtime.
//! Once that is set up and running, all windows can be constructed and played around with.
//! To do this, you use `Application::initialize`.
//! Then you have an `Application` instance, from which you can obtain a new `Runtime` instance.
//! Running it will grant you access to an application handle which you can manage the application with, and from which you can create all your windows with.
//!
//! # Example #1
//! Here is an example to show how you can construct your application:
//! ```rust
//! use browser_window::application::*;
//!
//! fn main() {
//! 	let application = Application::initialize();
//! 	let runtime = application.start();
//!
//!      runtime.run_async(|app| async move {
//!
//!         // Do something ...
//!     });
//! }
//! ```
//!
//! # Example #2
//! If you want to run another kind of runtime, like (tokio)[https://tokio.rs/] for example, its still possible to use Browser Window in conjunction with it.
//! Here is an example:
//! ```rust
//! use browser_window::application::*;
//! use tokio;
//!
//! async fn alternative_main( app: ApplicationHandleThreaded ) {
//! 	// Do something...
//! }
//!
//! fn main() {
//! 	let application = Application::initialize();
//!
//! 	let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
//!     let bw_runtime = application.start();
//!     runtime.run(|_app| {
//!         let app: ApplicationHandleThreaded = _app.into();
//!
//! 		tokio_runtime.spawn( alternative_main( app ) );
//! 	});
//! }
//! ```

use browser_window_ffi::*;
use lazy_static::lazy_static;
use std::env;
use std::ffi::{c_void, CString};
use std::future::Future;
use std::os::raw::{c_int};
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

use super::common::*;


/// Use this to initialize and start your application with.
pub struct Application {
	pub(in super) handle: ApplicationHandle
}

/// A thread-safe application handle.
/// This handle also allows you to dispatch code to be executed on the GUI thread from any other thread.
#[derive(Clone, Copy)]
pub struct ApplicationHandleThreaded {
	pub(in super) handle: ApplicationHandle
}
unsafe impl Send for ApplicationHandleThreaded {}
unsafe impl Sync for ApplicationHandleThreaded {}

#[derive(Clone, Copy)]
/// A thread-unsafe application handle.
/// Often provided by for Browser Window.
pub struct ApplicationHandle {
	pub(in super) ffi_handle: *mut bw_Application
}

struct ApplicationDispatchData<'a> {

	handle: ApplicationHandle,
	func: Box<dyn FnOnce(ApplicationHandle) + Send + 'a>
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

	/// In order to use the Browser Window API, you need to initialize Browser Window at the very start of your application.
	/// Preferably on the first line of your `main` function.
	///
	/// # Warning
	/// This will open another process of your application.
	/// Therefore, any code that will be placed before initialization will also be executed on all other processes.
	/// This is generally unnecessary.
	pub fn initialize() -> Application {

		let (args_vec, mut ptrs_vec) = Self::args_ptr_vec();
		let argc: c_int = args_vec.len() as _;
		let argv = ptrs_vec.as_mut_ptr();

		let ffi_handle = unsafe { bw_Application_initialize( argc, argv as _ ) };

		Application::from_ffi_handle( ffi_handle )
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
		unsafe { bw_Application_free( self.handle.ffi_handle ) };
	}
}

impl Runtime {

	/// Polls a future given a pointer to the waker data.
	unsafe fn poll_future( data: *mut WakerData ) {
		debug_assert!( data != ptr::null_mut(), "WakerData pointer can't be zero!" );
		// Test if polling from the right thread
		#[cfg(debug_assertions)]
		bw_Application_assertCorrectThread( (*data).handle.ffi_handle );

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

	/// Run the main loop.
	/// This is useful if you want to manipulate the GUI from other threads.
	///
	/// # Arguments
	/// * `on_ready` - This closure will be called when the runtime has initialized, and will provide an application handle.
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

	/// Spawns the given future, executing it on the GUI thread somewhere in the near future.
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

		unsafe {
			let exit_code = bw_Application_run( self.handle.ffi_handle, ffi_ready_handler::<H>, ready_data as _ );
			return exit_code;
		}
	}
}



impl Application {

	/// Constructs an `Application` from a ffi handle
	pub(in super) fn from_ffi_handle( ffi_handle: *mut bw_Application ) -> Self {
		Self {
			handle: ApplicationHandle::new( ffi_handle )
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

	/// Causes the `Runtime` to terminate.
	/// The `Runtime`'s run or spawn command will return the exit code provided.
	/// This will mean that not all tasks might complete.
	/// If you were awaiting
	pub fn exit( &self, exit_code: i32 ) {
		unsafe { bw_Application_exit( self.ffi_handle, exit_code as _ ); }
	}

	pub(in super) fn new( ffi_handle: *mut bw_Application ) -> Self {
		Self {
			ffi_handle: ffi_handle
		}
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



impl ApplicationHandleThreaded {

	/// Executes the given closure `func` on the GUI thread, and gives back the result when done.
	/// This only works when the runtime is still running.
	/// If the closure panicked, or the runtime is not running, this will return an error.
	///
	/// The function signature is practically the same as:
	/// ```rust
	/// pub async fn delegate<'a,F,R>( &self, func: F ) -> Result<R, DelegateError> where
	/// 	F: FnOnce( ApplicationHandle ) -> R + Send + 'a,
	/// 	R: Send { /* ... */ }
	/// ```
	///
	/// Keep in mind that in multi-threaded environments, it is generally a good idea to put the output on the heap.
	/// The output value _will_ be copied.
	///
	/// # Example
	/// ```rust
	/// let my_value: String = app.delegate(|handle| {
	/// 	"String".to_owned()
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
	/// ```rust
	/// pub async fn delegate_future<'a,F,R>( &self, func: F ) -> Result<R, DelegateError> where
	/// 	F: Future<Output=R> + 'static,,
	/// 	R: Send { /* ... */ }
	/// ```
	///
	/// # Example
	/// ```rust
	/// let my_value: String = app.delegate_future(async {
	/// 	"String".to_owned()
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
	/// ```rust
	/// |handle| async move { /* ... */ }
	/// ```
	///
	/// The function signature is practically the same as:
	/// ```rust
	/// pub async fn delegate_async<'a,C,F,R>( &self, func: C ) -> Result<R, DelegateError> where
	/// 	C: FnOnce( ApplicationHandle ) -> F + Send + 'a,
	/// 	F: Future<Output=R>,
	/// 	R: Send + 'static
	/// { /* ... */ }
	/// ```
	///
	/// # Example
	/// ```rust
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
		let data = Box::into_raw( Box::new( ApplicationDispatchData {
			handle: self.handle,
			func: Box::new( func )
		} ) );

		unsafe {
			bw_Application_dispatch(
				self.handle.ffi_handle,
				ffi_dispatch_handler,
				data as _
			)
		}
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
		unsafe { bw_Application_exitAsync( self.handle.ffi_handle, exit_code as _ ); }
	}

	/// Constructs an `ApplicationThreaded` handle from a ffi handle
	pub(in super) fn from_ffi_handle( ffi_handle: *mut bw_Application ) -> Self {
		Self {
			handle: ApplicationHandle::new( ffi_handle )
		}
	}

	/// Executes the given future on the GUI thread somewhere in the near future.
	pub fn spawn<F>( &self, future: F ) where
		F: Future<Output=()> + 'static
	{
		self.handle.spawn( future );
	}
}

impl From<ApplicationHandle> for ApplicationHandleThreaded {
	fn from( other: ApplicationHandle ) -> Self {
		Self {
			handle: other.clone()
		}
	}
}



impl HasAppHandle for ApplicationHandle {

	fn app_handle( &self ) -> ApplicationHandle {
		self.clone()
	}
}



unsafe extern "C" fn ffi_dispatch_handler(_app: *mut bw_Application, _data: *mut c_void ) {

	let data_ptr = _data as *mut ApplicationDispatchData<'static>;
	let data = Box::from_raw( data_ptr );

	(data.func)( data.handle.into() );
}

/// The handler that is invoked when the runtime is deemed 'ready'.
unsafe extern "C" fn ffi_ready_handler<H>( ffi_handle: *mut bw_Application, user_data: *mut c_void ) where
	H: FnOnce( ApplicationHandle )
{
	let app = ApplicationHandle::new( ffi_handle );
	let closure = Box::from_raw( user_data as *mut H );

	closure( app );
}

/// A handler that is invoked by wakers.
unsafe extern "C" fn ffi_wakeup( _ffi_handle: *mut bw_Application, user_data: *mut c_void ) {

	let	data = user_data as *mut WakerData;

	Runtime::poll_future( data );
}

unsafe fn waker_clone( data: *const () ) -> RawWaker {
	RawWaker::new( data, &WAKER_VTABLE )
}

unsafe fn waker_wake( data: *const () ) {
	let data_ptr = data as *const WakerData;

	bw_Application_dispatch(
		(*data_ptr).handle.ffi_handle,
		ffi_wakeup,
		data_ptr as _
	);
}

unsafe fn waker_wake_by_ref( data: *const () ) {
	waker_wake( data );
}

fn waker_drop( _data: *const () ) {}
