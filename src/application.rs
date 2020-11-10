use browser_window_ffi::*;
use std::env;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

use super::common::*;



/// An handle for this application.
/// Can be seen as an interface for the Application and ApplicationAsync 'handles'.
#[derive(Clone)]
pub struct Application {
	pub(in super) handle: ApplicationHandle,
	_not_send: PhantomData<Rc<()>>
}

/// A thread-safe application handle.
/// This handle also allows you to dispatch code to be executed on the GUI thread.
pub struct ApplicationAsync {
	pub(in super) handle: ApplicationHandle
}
unsafe impl Sync for ApplicationAsync {}

/// The `ApplicationHandle` i
#[derive(Clone)]
pub struct ApplicationHandle {
	pub(in super) ffi_handle: *mut bw_Application
}
// ApplicationHandle needs to be Send and Sync because internally it is so widely used across async code,
//  that the constraints are unworkable.
unsafe impl Send for ApplicationHandle {}

/// Use this to start and run the application with.
pub struct Runtime {
	pub(in super) handle: ApplicationHandle
}



/// The future that dispatches a closure onto the GUI thread
pub type ApplicationDispatchFuture<'a,R> = DispatchFuture<'a, ApplicationHandle, R>;



impl Runtime {

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

	/// Run the main loop.
	/// This method finishes when all windows are closed, or when the application has been signalled to exit.
	/// When ready, the provided closure will be called, which will be given an application handle that can be used to create browser windows.
	///
	/// # Arguments
	/// * `on_ready` - This closure will be called when all setup work is completed.
	pub fn run<H>( self, on_ready: H ) -> i32 where
		H: FnOnce( Application )
	{
		let ready_data = Box::into_raw( Box::new( on_ready ) );

		unsafe {
			let exit_code = bw_Application_run( self.handle.ffi_handle, ffi_ready_handler::<H>, ready_data as _ );
			bw_Application_finish( self.handle.ffi_handle );
			return exit_code;
		}
	}

	/// Run the main loop synchronously, despite what the name might suggest.
	/// This basically does the same as `Runtime::run`, but provides an thread-safe application handle when ready.
	/// This is useful if you want to manipulate the GUI from other threads.
	///
	/// # Arguments
	/// * `on_ready` - This closure will be called when all setup work is completed.
	pub fn run_alongside<H>( self, on_ready: H ) -> i32 where
		H: FnOnce( ApplicationAsync )
	{
		let ready_data = Box::into_raw( Box::new( on_ready ) );

		unsafe {
			let exit_code = bw_Application_run( self.handle.ffi_handle, ffi_ready_async_handler::<H>, ready_data as _ );
			bw_Application_finish( self.handle.ffi_handle );
			return exit_code;
		}
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
	pub fn into_async( self ) -> ApplicationAsync {
		ApplicationAsync {
			handle: self.handle
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



impl ApplicationAsync {

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

	/// Constructs an `ApplicationAsync` handle from a ffi handle
	pub(in super) fn from_ffi_handle( ffi_handle: *mut bw_Application ) -> Self {
		Self {
			handle: ApplicationHandle::new( ffi_handle )
		}
	}
}

impl Deref for ApplicationAsync {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&self.handle
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



extern "C" fn ffi_ready_handler<H>( ffi_handle: *mut bw_Application, user_data: *mut c_void ) where
	H: FnOnce( Application )
{

	let app = Application::from_ffi_handle( ffi_handle );
	let closure = unsafe { Box::from_raw( user_data as *mut Box<H> ) };

	closure( app );
}

extern "C" fn ffi_ready_async_handler<H>( ffi_handle: *mut bw_Application, user_data: *mut c_void ) where
	H: FnOnce( ApplicationAsync )
{
	let app = ApplicationAsync::from_ffi_handle( ffi_handle );
	let closure = unsafe { Box::from_raw( user_data as *mut H ) };

	closure( app );
}
