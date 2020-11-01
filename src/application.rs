use browser_window_ffi::*;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use super::common::*;



/// A thread-unsafe handle to an application instance.
#[derive(Clone)]
pub struct Application {
	pub inner: Rc<ApplicationInner>
}



/// A thread-safe application handle.
/// This handle also allows you to dispatch code to be executed on the GUI thread.
#[derive(Clone)]
pub struct ApplicationAsync {
	pub inner: Arc<ApplicationInner>
}



/// An application handle that can not be instantiated,
///     but is provided by certain handlers
#[derive(Clone)]
pub struct ApplicationHandle {
	pub _ffi_handle: *mut bw_Application
}
unsafe impl Send for ApplicationHandle {}
unsafe impl Sync for ApplicationHandle {}

/// The future that dispatches a closure onto the GUI thread
pub type ApplicationDispatchFuture<'a,R> = DispatchFuture<'a, ApplicationHandle, R>;

pub struct ApplicationInner {
	pub inner: ApplicationHandle
}



impl Application {

	/// Get an async clone of this handle
	pub fn into_async( self ) -> ApplicationAsync {
		
		// Convert an Rc to an Arc
		let inner = unsafe { Arc::from_raw( Rc::into_raw( self.inner ) ) };
		
		ApplicationAsync {
			inner: inner
		}
	}

	/// Constructs a new application handle
	/// Only call this once
	pub fn new() -> Self {
		let ffi_handle = unsafe { bw_Application_new() };

		Self {
			inner: Rc::new( ApplicationInner{
				inner: ApplicationHandle {
					_ffi_handle: ffi_handle
				}
			} )
		}
	}

	/// Run the main loop
	/// This method finishes when all windows are closed.
	pub fn run( &self ) -> i32 {
		unsafe { bw_Application_run( self.inner._ffi_handle ) }
	}
}

impl Deref for Application {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&**self.inner
	}
}

impl AppHandle for ApplicationHandle {
	fn app_handle( &self ) -> ApplicationHandle {
		ApplicationHandle {
			_ffi_handle: self._ffi_handle
		}
	}
}

impl ApplicationAsync {

	/// Clones the async version of this application handle into a non-async version.
	/// This is unsafe because the non-async version of the handle may only be used on the thread it was created on,
	///  while this method might not have been called on that thread
	unsafe fn clone_threadunsafe_handle( &self ) -> ApplicationHandle {
		ApplicationHandle {
			_ffi_handle: (**self.inner)._ffi_handle.clone()
		}
	}

	/// Executes the given closure on the GUI thread.
	pub fn dispatch<'a,F,R>( &self, func: F ) -> ApplicationDispatchFuture<'a,R> where
		F: FnOnce( ApplicationHandle ) -> R + Send + 'a,
		R: Send
	{
		ApplicationDispatchFuture::<'a,R>::new( unsafe { self.clone_threadunsafe_handle() }, func )
	}

	/// Signals the application to exit.
	/// The run command will return the exit code provided.
	pub fn exit( &self, exit_code: u32 ) {
		// The thread-safe version of bw_Application_exit:
		unsafe { bw_Application_exitAsync( self.inner._ffi_handle, exit_code as _ ); }
	}
}

impl From<Application> for ApplicationAsync {
	fn from( app: Application ) -> Self {
		app.into_async()
	}
}



impl ApplicationHandle {

	pub fn exit( &self, exit_code: u32 ) {
		unsafe { bw_Application_exit( self._ffi_handle, exit_code as _ ); }
	}

	/// Constructs an ApplicationHandle from an internal C handle
	pub fn from_ptr( ptr: &mut bw_Application ) -> ApplicationHandle {
		ApplicationHandle {
			_ffi_handle: ptr
		}
	}
}



impl Deref for ApplicationInner {
	type Target = ApplicationHandle;

	fn deref( &self ) -> &Self::Target {
		&self.inner
	}
}

impl Drop for ApplicationInner {
	fn drop( &mut self ) {
		unsafe { bw_Application_free( self._ffi_handle ) }
	}
}
