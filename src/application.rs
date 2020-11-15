use browser_window_ffi::*;
use lazy_static::lazy_static;
use std::env;
use std::ffi::{c_void, CString};
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::{c_int};
use std::pin::Pin;
use std::ptr;
use std::rc::Rc;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

use super::common::*;


/// An handle for this application.
#[derive(Clone, Copy)]
pub struct Application {
	pub(in super) handle: ApplicationHandle,
	_not_send: PhantomData<Rc<()>>
}

/// A thread-safe application handle.
/// This handle also allows you to dispatch code to be executed on the GUI thread.
#[derive(Clone, Copy)]
pub struct ApplicationThreaded {
	pub(in super) handle: ApplicationHandle
}
unsafe impl Sync for ApplicationThreaded {}

#[derive(Clone, Copy)]
pub struct ApplicationHandle {
	pub(in super) ffi_handle: *mut bw_Application
}
// # Safety
// `ApplicationHandle` is Send because it is used extensively by `ApplicationThreaded`,
//  which only uses the handle with thread-safe functions.
unsafe impl Send for ApplicationHandle {}

struct ApplicationDispatchData<'a> {

	handle: ApplicationHandle,
	func: Box<dyn FnOnce(Application) + Send + 'a>
}

/// Use this to start and run the application with.
pub struct Runtime {
	pub(in super) handle: ApplicationHandle
}

/// The data that is available to a waker, allowing it to poll a future.
struct WakerData {
	handle: ApplicationHandle,
	future: Pin<Box<dyn Future<Output=()>>>
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



impl Runtime {

	/// Obtains an application handle for this runtime.
	pub fn app( &self ) -> Application {
		self.handle.clone().into()
	}

	/// Obtains an thread-safe application handle for this runtime.
	pub fn app_threaded( &self ) -> ApplicationThreaded {
		self.handle.clone().into()
	}

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

	/// Polls a future given a pointer to the waker data.
	unsafe fn poll_future( data: *mut WakerData ) {
		debug_assert!( data != ptr::null_mut(), "WakerData pointer can't be zero!" );

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
			on_ready( handle.into() )
		} )
	}

	/// Runs the main loop and executes the given future within that loop.
	/// This function exits when the future finishes or when `exit` is called.
	///
	/// # Reserved Codes
	/// The same reserved codes apply as `run`.
	// TODO: Turn this future into an async closure.
	pub fn run_async<F>( &self, future: F ) -> i32 where
		F: Future<Output=()> + 'static
	{
		self._run(|handle| {
			handle.spawn( future );
		})
	}

	/// Starts the GUI application.
	/// Only call this once, and at the start of your application, before anything else is done.
	/// Everything that runs before this function, runs as well on the other (browser engine related) processes.
	/// That is generally unnecessary.
	pub fn start() -> Self {
		let (args_vec, mut ptrs_vec) = Self::args_ptr_vec();
		let argc: c_int = args_vec.len() as _;
		let argv = ptrs_vec.as_mut_ptr();

		let ffi_handle = unsafe { bw_Application_start( argc, argv as _ ) };

		Self {
			handle: ApplicationHandle::new( ffi_handle )
		}
	}

	fn _run<H>( &self, on_ready: H ) -> i32 where
		H: FnOnce( ApplicationHandle )
	{
		let ready_data = Box::into_raw( Box::new( on_ready ) );

		unsafe {
			let exit_code = bw_Application_run( self.handle.ffi_handle, ffi_ready_handler::<H>, ready_data as _ );
			bw_Application_finish( self.handle.ffi_handle );
			return exit_code;
		}
	}
}



impl Application {

	/// Causes the `Runtime` to terminate.
	/// The `Runtime`'s run or spawn command will return the exit code provided.
	/// This will mean that not all tasks might complete.
	/// If you were awaiting
	pub fn exit( &self, exit_code: i32 ) {
		unsafe { bw_Application_exit( self.handle.ffi_handle, exit_code as _ ); }
	}

	/// Constructs an `Application` from a ffi handle
	pub(in super) fn from_ffi_handle( ffi_handle: *mut bw_Application ) -> Self {
		Self {
			handle: ApplicationHandle::new( ffi_handle ),
			_not_send: PhantomData
		}
	}
}

impl Deref for Application {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&self.handle
	}
}

impl From<ApplicationHandle> for Application {
	fn from( other: ApplicationHandle ) -> Self {
		Self {
			handle: other,
			_not_send: PhantomData
		}
	}
}



impl ApplicationHandle {

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



impl ApplicationThreaded {

	/// Executes the given closure `func` on the GUI thread, and gives back the result when done.
	/// This only works when the runtime is still running.
	/// If the closure panicked, or the runtime is not running, this will return an error.
	///
	/// Keep in mind that in multi-threaded environments, it is generally a good idea to put the output on the heap.
	/// The output value _will_ be copied.
	pub fn delegate<'a,F,R>( &self, func: F ) -> ApplicationDelegateFuture<'a,R> where
		F: FnOnce( Application ) -> R + Send + 'a,
		R: Send
	{
		ApplicationDelegateFuture::<'a,R>::new( self.handle.clone(), |handle| {
			func( handle.into() )
		} )
	}

	/// Executes the given `future` on the GUI thread, and gives back the output when done.
	/// This only works when the runtime is still running.
	/// If the future panicked during a poll, or the runtime is not running, this will return an error.
	///
	/// See also `delegate`.
	pub fn delegate_future<F,R>( &self, future: F ) -> DelegateFutureFuture<R> where
		F: Future<Output=R> + 'static,
		R: Send + 'static
	{
		DelegateFutureFuture::new( self.handle.clone(), future )
	}

	/// Executes the given async closure `func` on the GUI thread, and gives back the result when done.
	/// This only works when the runtime is still running.
	/// If the closure panicked, or the runtime is not running, this will return an error.
	pub fn delegate_async<'a,C,F,R>( &self, func: C ) -> DelegateFutureFuture<'a,R> where
		C: FnOnce( Application ) -> F + Send + 'a,
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
		F:  FnOnce( Application ) + Send + 'a
	{
		let data = Box::into_raw( Box::new( ApplicationDispatchData {
			handle: self.handle,
			func: Box::new( func )
		} ) );

		unsafe {
			bw_Application_dispatch(
				self.handle.ffi_handle,
				ffi_application_dispatch_handler,
				data as _
			)
		}
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

	pub fn spawn<F>( &self, future: F ) where
		F: Future<Output=()> + 'static
	{
		self.handle.spawn( future );
	}
}

impl Deref for ApplicationThreaded {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&self.handle
	}
}

impl From<ApplicationHandle> for ApplicationThreaded {
	fn from( other: ApplicationHandle ) -> Self {
		Self {
			handle: other
		}
	}
}



impl HasAppHandle for ApplicationHandle {

	fn app_handle( &self ) -> ApplicationHandle {
		self.clone()
	}
}



unsafe extern "C" fn ffi_application_dispatch_handler( _app: *mut bw_Application, _data: *mut c_void ) {

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
