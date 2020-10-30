use boxfnonce::SendBoxFnOnce;
use browser_window_ffi::*;
use std::{
	error::Error,
	ffi::CStr,
	ops::Deref,
	os::raw::*,
	rc::Rc,
	sync::Arc
};
use tokio::sync::oneshot;

use crate::application::{
	ApplicationHandle
};
use crate::common::*;



/// A thread-unsafe handle to a browser window.
/// A reference counter is held internally,
///     meaning that you can simply clone this handle without having to worry about memory leakage.
/// If the user closes the window, this handle stays valid.
/// Also, if you lose this handle, window destruction and cleanup is only done when the user actually closes it.
/// So you don't have to worry about lifetimes and/or propper destruction of the window either.
#[derive(Clone)]
pub struct BrowserWindow {
	pub inner: Rc<BrowserWindowInner>
}

/// A thread-safe handle to a browser window.
/// It provides the same functionality as Browserwindow.
/// However, each function is async: it runs on the GUI thread, and returns when it is done.
/// Also, it allows you to dispatch a closure on that thread.
#[derive(Clone)]
pub struct BrowserWindowAsync {
	pub inner: Arc<BrowserWindowInner>
}

#[derive(Clone)]
pub struct BrowserWindowHandle {
	pub _ffi_handle: *mut bw_BrowserWindow
}
// We implement Send and Sync of BrowserWindowHandle because it is used internally by BrowserWindowAsync as well.
unsafe impl Send for BrowserWindowHandle {}
unsafe impl Sync for BrowserWindowHandle {}

/// This structure holds an application handle and a browser window handle.
/// The purpose of this structure is to invoke the FFI function to drop the browser window handle, when this struct is dropped naturally by Rust.
/// So by putting this struct in an Arc<...>, you effectively have some sort garbage collection.
pub struct BrowserWindowInner {
	pub app: ApplicationHandle,
	pub handle: BrowserWindowHandle	// TODO: Change name to "browser", handle is too ambigious
}



impl AppHandle for BrowserWindow {
	fn app_handle( &self ) -> ApplicationHandle {
		self.inner.app.clone()
	}
}



impl BrowserWindow {

	pub fn into_async( self ) -> BrowserWindowAsync {
		// Convert a Rc to an Arc
		let inner = unsafe { Arc::from_raw( Rc::into_raw( self.inner ) ) };

		BrowserWindowAsync {
			inner: inner
		}
	}
}

impl Deref for BrowserWindow {
	type Target = BrowserWindowHandle;

	fn deref( &self ) -> &Self::Target {
		&self.inner.handle
	}
}



impl BrowserWindowAsync {

	/// Closes the browser.
	/// The browser will be freed from memory when the last handle to it gets dropped.
	pub async fn close( self ) {
		self.dispatch(|bw| {
			bw.close()
		}).await;
	}

	/// Executes the given closure within the GUI thread
	///
	/// # Arguments
	/// * `func` - The closure to run on the GUI thread
	pub fn dispatch<'a,F,R>( &self, func: F ) -> BrowserWindowDispatchFuture<'a,R> where
		F: FnOnce( BrowserWindowHandle ) -> R + Send + 'a,
		R: Send
	{
		BrowserWindowDispatchFuture::new( self.inner.handle.clone(), func )
	}

	/// Executes the given javascript code, and returns the resulting output as a string when done.
	///
	/// # Arguments:
	/// * `js` - Javascript code
	pub async fn eval_js( &self, js: &str ) -> Result<String, Box<dyn Error + Send>> {

		let (tx, rx) = oneshot::channel::<Result<String, Box<dyn Error + Send>>>();

		self.dispatch(move |bw| {

			// Executing the JavaScript on the GUI thread
			bw.eval_js( js, move |_, result| {

				// Result is ready
				tx.send( result ).unwrap();
			} );
		}).await;

		// The result
		rx.await.unwrap()
	}

	/// Causes the browser to navigate to the given url.
	///
	/// # Arguments
	/// * `url` - The url to navigate to
	pub async fn navigate( &self, url: &str ) -> Result<(), Box<dyn Error + Send>> {
		*self.dispatch(|bw| {
			bw.navigate( url )
		}).await
	}
}



type BrowserWindowCallbackData<'a> = SendBoxFnOnce<'a,(BrowserWindowHandle, Result<String, Box<dyn Error + Send>>),()>;

pub type BrowserWindowDispatchFuture<'a,R> = DispatchFuture<'a, BrowserWindowHandle, R>;



impl BrowserWindowHandle {

	/// Closes the browser.
	/// The browser will be freed from memory when the last handle to it gets dropped.
	pub fn close( self ) {
		unsafe { bw_BrowserWindow_close( self._ffi_handle ); }
	}

	/// Executes the given javascript code, and returns the output via a callback.
	/// If you don't need the result, see "exec_js".
	///
	/// # Arguments:
	/// * `js` - The javascript code to execute.
	/// * `on_complete` - The 'callback'. This closure will be invoked, with the result provided as the first argument.
	///                   The result contains the output of the javascript code when it succeeded.
	///                   Otherwise the error explains the javascript exception.
	pub fn eval_js<'a,H>( &self, js: &str, on_complete: H ) where
		H: FnOnce( BrowserWindowHandle, Result<String, Box<dyn Error + Send>> ) + Send + 'a
	{
		let data_ptr = Box::into_raw( Box::new(
			BrowserWindowCallbackData::<'a>::new( on_complete )
		) );

		unsafe { bw_BrowserWindow_eval_js(
			self._ffi_handle,
			js.into(),
			ffi_eval_js_callback,
			data_ptr as _
		) };
	}

	/// Executes the given javascript code without waiting on it to finish.
	///
	/// # Arguments:
	/// * `js` - The javascript code
	pub fn exec_js( &self, js: &str ) {
		self.eval_js( js, |_,_|{} );
	}

	pub unsafe fn from_ptr( ptr: *mut bw_BrowserWindow ) -> Self {
		Self {
			_ffi_handle: ptr
		}
	}

	/// Causes the browser to navigate to the given url.
	///
	/// # Arguments
	/// * `url` - The url to navigate to
	pub fn navigate( &self, url: &str ) -> Result<(), Box<dyn Error + Send>> {
		let err = unsafe { bw_BrowserWindow_navigate( self._ffi_handle, url.into() ) };

		if err.code == 0 {
			return Ok(());
		}

		Err( Box::new( err ) )
	}
}

impl AppHandle for BrowserWindowHandle {
	fn app_handle( &self ) -> ApplicationHandle {
		ApplicationHandle {
			_ffi_handle: unsafe { bw_BrowserWindow_get_app( self._ffi_handle ) as _ }
		}
	}
}



impl Deref for BrowserWindowInner {
	type Target = BrowserWindowHandle;

	fn deref( &self ) -> &Self::Target {
		&self.handle
	}
}

impl Drop for BrowserWindowInner {
	fn drop( &mut self ) {
		unsafe { bw_BrowserWindow_drop( self.handle._ffi_handle ) }
	}
}



extern "C" fn ffi_eval_js_callback( bw: *mut bw_BrowserWindow, cb_data: *mut c_void, result: *const c_char, error: *const bw_Err ) {

	let data_ptr = cb_data as *mut BrowserWindowCallbackData;
	let data = unsafe { Box::from_raw( data_ptr ) };

	// Construct a result value depending on whether the result or error parameters are set
	let result_val: Result<String, Box<dyn Error + Send>> = if error.is_null() {
		let result_str = unsafe { CStr::from_ptr( result ).to_string_lossy().to_owned().to_string() };
		Ok( result_str )
	}
	else {
		Err( Box::new( unsafe { (*error).clone() } ) )
	};

	// Construct a
	let handle = unsafe { BrowserWindowHandle::from_ptr( bw ) };

	data.call( handle, result_val );
}
