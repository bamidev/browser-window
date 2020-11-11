use browser_window_ffi::*;
use lazy_static::lazy_static;
use std::env;
use std::ffi::c_void;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::{c_char, c_int};
use std::pin::Pin;
use std::ptr;
use std::rc::Rc;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

use super::common::*;



/// An handle for this application.
/// Can be seen as an interface for the `Application` and `ApplicationThreaded` handles.
#[derive(Clone)]
pub struct Application {
	pub(in super) handle: ApplicationHandle,
	_not_send: PhantomData<Rc<()>>
}

/// A thread-safe application handle.
/// This handle also allows you to dispatch code to be executed on the GUI thread.
pub struct ApplicationThreaded {
	pub(in super) handle: ApplicationHandle
}
unsafe impl Sync for ApplicationThreaded {}

/// The `ApplicationHandle` i
#[derive(Clone)]
pub struct ApplicationHandle {
	pub(in super) ffi_handle: *mut bw_Application
}
// # Safety
// `ApplicationHandle` is Send because it is used extensively by `ApplicationThreaded`,
//  which only uses the handle with thread-safe functions.
unsafe impl Send for ApplicationHandle {}

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
pub type ApplicationDispatchFuture<'a,R> = DispatchFuture<'a, ApplicationHandle, R>;



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

	pub fn app( &self ) -> Application {
		self.handle.clone().into()
	}

	fn args_ptr_vec() -> Vec<*mut c_char> {
		let args = env::args_os();
		let mut vec = Vec::with_capacity( args.len() );

		for arg in args {
			vec.push(
				arg
					.as_os_str()
					.to_str()
					.expect("Invalid Unicode in console arguments!")
					.as_ptr()
					 as _
			);
		}

		vec
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
	pub fn run<H>( &self, on_ready: H ) -> i32 where
		H: FnOnce( ApplicationThreaded )
	{
		return self._run( |handle| {
			on_ready( handle.into() )
		} )
	}

	/// Runs the main loop and executes the given future within that loop.
	/// Use this when you are fine with running Browser Window single-threaded.
	pub fn spawn<F>( &self, future: F ) -> i32 where
		F: Future<Output=()> + 'static
	{
		self._run(|handle| {

			// Data for the waker.
			let waker_data = Box::into_raw( Box::new(
				WakerData {
					handle: handle,
					future: Box::pin( future )
				}
			) );

			// First poll
			unsafe { Runtime::poll_future( waker_data ) };
		})
	}

	/// Starts the GUI application.
	/// Only call this once, and at the start of your application, before anything else is done.
	/// Everything that runs before this function, runs as well on the other (browser engine related) processes.
	/// That is generally unnecessary.
	pub fn start() -> Self {
		let mut args_vec = Self::args_ptr_vec();
		let argc: c_int = args_vec.len() as _;
		let argv = args_vec.as_mut_ptr();

		let ffi_handle = unsafe { bw_Application_start( argc, argv ) };

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

	/// Signals the application to exit.
	/// The run command will return the exit code provided.
	///
	/// # Arguments
	/// * `exit_code` - The code that will be returned by the run function when it stops.
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

	/// Transforms the application handle into a asynchronous one.
	pub fn into_async( self ) -> ApplicationThreaded {
		ApplicationThreaded {
			handle: self.handle
		}
	}

	pub fn spawn<F>( &self, future: F ) where
		F: Future<Output=()> + 'static
	{
		// Create a context with our own waker
		let waker_data = Box::into_raw( Box::new(
			WakerData {
				handle: self.handle.clone(),
				future: Box::pin( future )
			}
		) );

		// First poll
		unsafe { Runtime::poll_future( waker_data ) };
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



impl ApplicationThreaded {

	/// Executes the given closure on the GUI thread.
	pub fn dispatch<'a,F,R>( &self, func: F ) -> ApplicationDispatchFuture<'a,R> where
		F: FnOnce( Application ) -> R + Send + 'a,
		R: Send
	{
		ApplicationDispatchFuture::<'a,R>::new( self.handle.clone(), |handle| {
			func( handle.into() )
		} )
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
		// Create a context with our own waker
		let waker_data = Box::into_raw( Box::new(
			WakerData {
				handle: self.handle.clone(),
				future: Box::pin( future )
			}
		) );

		// First poll
		unsafe { Runtime::poll_future( waker_data ) };
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



impl ApplicationHandle {
	pub(in super) fn new( ffi_handle: *mut bw_Application ) -> Self {
		Self {
			ffi_handle: ffi_handle
		}
	}
}

impl HasAppHandle for ApplicationHandle {

	fn app_handle( &self ) -> ApplicationHandle {
		self.clone()
	}
}



unsafe extern "C" fn ffi_ready_handler<H>( ffi_handle: *mut bw_Application, user_data: *mut c_void ) where
	H: FnOnce( ApplicationHandle )
{
	let app = ApplicationHandle::new( ffi_handle );
	let closure = Box::from_raw( user_data as *mut H );

	closure( app );
}

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
