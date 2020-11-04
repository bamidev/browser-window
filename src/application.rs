use browser_window_ffi::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use super::common::*;



/// A thread-unsafe handle to an application instance.
/// Use this to start the application with.
#[derive(Clone)]
pub struct Application {
	pub(in super) inner: Arc<ApplicationInner>,
	/// This field is purely here to force Application in not being Send or Sync
	_not_send: PhantomData<Rc<u8>>
}



/// A thread-safe application handle.
/// This handle also allows you to dispatch code to be executed on the GUI thread.
#[derive(Clone)]
pub struct ApplicationAsync {
	pub(in super) inner: Arc<ApplicationInner>
}



/// An handle for this application.
/// Can be seen as an interface for the Application and ApplicationAsync 'handles'.
#[derive(Clone)]
pub struct ApplicationHandle {
	pub(in super) _ffi_handle: *mut bw_Application
}
unsafe impl Send for ApplicationHandle {}
unsafe impl Sync for ApplicationHandle {}



/// The future that dispatches a closure onto the GUI thread
pub type ApplicationDispatchFuture<'a,R> = DispatchFuture<'a, ApplicationHandle, R>;

pub struct ApplicationInner {
	pub(in super) handle: ApplicationHandle
}



impl Application {

	/// Transform this handle into a thread-safe handle.
	pub fn into_async( self ) -> ApplicationAsync {
		ApplicationAsync {
			inner: self.inner
		}
	}

	/// Run the main loop.
	/// This method finishes when all windows are closed.
	pub fn run( &self ) -> i32 {
		unsafe { bw_Application_run( self._ffi_handle ) }
	}

	/// Starts the GUI application.
	/// Only call this once, and at the start of your program, before anything else.
	/// Everything that runs before this function, runs as well on the other (browser engine related) processes.
	/// This is generally unnecessary.
	pub fn start() -> Self {
		let ffi_handle = unsafe { bw_Application_start() };

		Self {
			inner: Arc::new( ApplicationInner{
				handle: ApplicationHandle::from_ptr( ffi_handle )
			} ),
			_not_send: PhantomData
		}
	}
}

impl Deref for Application {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&**self.inner
	}
}

impl ApplicationAsync {

	/// Executes the given closure on the GUI thread.
	pub fn dispatch<'a,F,R>( &self, func: F ) -> ApplicationDispatchFuture<'a,R> where
		F: FnOnce( ApplicationHandle ) -> R + Send + 'a,
		R: Send
	{
		ApplicationDispatchFuture::<'a,R>::new( (**self).clone(), func )
	}

	/// Signals the application to exit.
	/// The run command will return the exit code provided.
	pub fn exit( &self, exit_code: i32 ) {
		// The thread-safe version of bw_Application_exit:
		unsafe { bw_Application_exitAsync( self.inner.handle._ffi_handle, exit_code as _ ); }
	}
}

impl Deref for ApplicationAsync {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&self.inner.handle
	}
}

impl From<Application> for ApplicationAsync {
	fn from( app: Application ) -> ApplicationAsync {
		app.into_async()
	}
}



impl ApplicationHandle {

	/// Causes the run function to exit.
	///
	/// # Arguments
	/// * `exit_code` - The code that will be returned by the run function when it stops.
	pub fn exit( &self, exit_code: i32 ) {
		unsafe { bw_Application_exit( self._ffi_handle, exit_code as _ ); }
	}

	// Constructs an ApplicationHandle from an internal C handle
	fn from_ptr( ptr: *mut bw_Application ) -> ApplicationHandle {
		ApplicationHandle {
			_ffi_handle: ptr
		}
	}
}

impl HasAppHandle for ApplicationHandle {
	fn app_handle( &self ) -> ApplicationHandle {
		self.clone()
	}
}



impl Deref for ApplicationInner {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&self.handle
	}
}

impl Drop for ApplicationInner {
	fn drop( &mut self ) {
		unsafe { bw_Application_free( self.handle._ffi_handle ); }
	}
}
