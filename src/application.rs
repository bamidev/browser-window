use browser_window_ffi::*;
use std::ops::Deref;
use std::sync::Arc;

use super::common::*;



/// A thread-unsafe handle to an application instance.
/// To do anything useful with the browser_window lib, you need to construct an Apllication,
///     preferably on the main thread. (Some future implementations might not work on any other threads.)
/// Then, you can spawn any browser window you like.
/// Then you need to call method run. This will execute the event loop and causes your browser windows to appear.
///
/// Note:
///     Due to the way the internal structure is reused for ApplicationAsync, we have had to declare its internal structure as Send and Sync.
///     However, Application is not supposed to be Send or Sync.
///     We can't disable Send and Sync for Application without a new feature that is only available in Rust nightly atm.
///     Therefore, if you want upmost safety, enable feature "nightly" for this crate
#[derive(Clone)]
pub struct Application {
	pub inner: Arc<ApplicationInner>
}
#[cfg(feature = "nightly")]
impl !Send for Application {}



/// A thread-safe application handle.
/// This handle allows you to dispatch code to be executed on the GUI thread.
/// It can also be used to exit the application from some other thread.
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

pub type ApplicationDispatchFuture<'a,R> = DispatchFuture<'a, ApplicationHandle, R>;

pub struct ApplicationInner {
	pub inner: ApplicationHandle
}



impl Application {

	pub fn async_clone( &self ) -> ApplicationAsync {
		ApplicationAsync {
			inner: self.inner.clone()
		}
	}

	pub fn new() -> Self {
		let ffi_handle = unsafe { bw_Application_new() };

		Self {
			inner: Arc::new( ApplicationInner{
				inner: ApplicationHandle {
					_ffi_handle: ffi_handle
				}
			} )
		}
	}

	/// Run the main loop
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
		unsafe { bw_Application_exit_async( self.inner._ffi_handle, exit_code as _ ); }
	}
}

impl From<Application> for ApplicationAsync {
	fn from( app: Application ) -> Self {
		ApplicationAsync {
			inner: app.inner
		}
	}
}



impl ApplicationHandle {

	pub fn exit( &self, exit_code: u32 ) {
		unsafe { bw_Application_exit( self._ffi_handle, exit_code as _ ); }
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
