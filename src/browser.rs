use boxfnonce::SendBoxFnOnce;
use browser_window_ffi::*;
use futures_channel::oneshot;
use std::{
	error::Error,
	ffi::CStr,
	marker::PhantomData,
	ops::Deref,
	os::raw::*,
	rc::Rc
};

use crate::application::*;
use crate::common::*;

pub mod builder;

pub use builder::BrowserBuilder;



//type BrowserJsCallbackData<'a> = Box<dyn FnOnce(Browser, Result<String, Box<dyn Error>>) + 'a>;
type BrowserJsAsyncCallbackData<'a> = SendBoxFnOnce<'a,(BrowserHandle, Result<String, Box<dyn Error + Send>>),()>;

/// The future that dispatches a closure on the GUI thread.
pub type BrowserDispatchFuture<'a,R> = DispatchFuture<'a, BrowserHandle, R>;



/// A thread-unsafe handle to a browser window
// If the user closes the window, this handle remains valid.
// Also, if you lose this handle, window destruction and cleanup is only done when the user actually closes it.
// So you don't have to worry about lifetimes and/or propper destruction of the window either.
pub struct Browser {
	pub(in super) handle: BrowserHandle,
	_not_send: PhantomData<Rc<()>>
}

/// A thread-safe handle to a browser window.
/// It allows you to dispatch code to the GUI thread.
// It provides the same functionality as `Browser`.
// However, each function is async: it runs on the GUI thread, and returns when it is done.
pub struct BrowserAsync {
	pub(in super) handle: BrowserHandle
}
unsafe impl Sync for BrowserAsync {}

#[derive(Clone)]
pub struct BrowserHandle {
	pub(in super) ffi_handle: *mut bw_BrowserWindow
}
unsafe impl Send for BrowserHandle {}



impl Browser {

	pub fn app( &self ) -> Application {
		Application::from_ffi_handle( unsafe { bw_BrowserWindow_getApp( self.handle.ffi_handle ) } )
	}

	/// Closes the browser.
	// The browser will be freed from memory when the last handle to it gets dropped.
	pub fn close( self ) {
		unsafe { bw_BrowserWindow_close( self.handle.ffi_handle ); }
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
		H: FnOnce( Browser, Result<String, Box<dyn Error>> ) + 'a
	{
		let data_ptr: *mut H = Box::into_raw(
			Box::new( on_complete )
		);

		unsafe { bw_BrowserWindow_evalJs(
			self.handle.ffi_handle,
			js.into(),
			ffi_eval_js_callback::<H>,
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

	fn from_ffi_handle( ptr: *mut bw_BrowserWindow ) -> Self {
		Self {
			handle: BrowserHandle::new( ptr ),
			_not_send: PhantomData
		}
	}

	/// Causes the browser to navigate to the given url.
	///
	/// # Arguments
	/// * `url` - The url to navigate to
	pub fn navigate( &self, url: &str ) -> Result<(), Box<dyn Error + Send>> {
		let err = unsafe { bw_BrowserWindow_navigate( self.handle.ffi_handle, url.into() ) };

		if err.code == 0 {
			return Ok(());
		}

		Err( Box::new( err ) )
	}
}

impl Deref for Browser {
	type Target = BrowserHandle;

	fn deref( &self ) -> &BrowserHandle {
		&self.handle
	}
}

impl Drop for Browser {
	fn drop( &mut self ) {
		unsafe { bw_BrowserWindow_drop( self.handle.ffi_handle ) }
	}
}

impl From<BrowserHandle> for Browser {

	fn from( handle: BrowserHandle ) -> Self {
		Self {
			handle: handle,
			_not_send: PhantomData
		}
	}
}

impl HasAppHandle for Browser {

	fn app_handle( &self ) -> ApplicationHandle {
		self.handle.app_handle()
	}
}



impl BrowserAsync {

	pub fn app( &self ) -> ApplicationAsync {
		ApplicationAsync::from_ffi_handle( unsafe { bw_BrowserWindow_getApp( self.handle.ffi_handle ) } )
	}

	/// Closes the browser.
	pub async fn close( self ) {
		self.dispatch(|bw| {
			bw.close()
		}).await;
	}

	/// Executes the given closure within the GUI thread, and return the value that the closure returned.
	/// Keep in mind that in multi-threaded environments, it is generally a good idea to Box return type,
	///  or use something else to put the value on the heap when dealing with large types.
	///
	/// # Arguments
	/// * `func` - The closure to run on the GUI thread.
	pub fn dispatch<'a,F,R>( &self, func: F ) -> BrowserDispatchFuture<'a,R> where
		F: FnOnce( Browser ) -> R + Send + 'a,
		R: Send
	{
		BrowserDispatchFuture::new( self.handle.clone(), |handle| {
			func( handle.into() )
		} )
	}

	/// Executes the given javascript code, and returns the resulting output as a string when done.
	///
	/// # Arguments:
	/// * `js` - Javascript code
	pub async fn eval_js( &self, js: &str ) -> Result<String, Box<dyn Error + Send>>
	{
		let (tx, rx) = oneshot::channel::<Result<String, Box<dyn Error + Send>>>();

		let handle = self.handle.clone();

		self.dispatch(move |_| {

			// Send the value back with the oneshot channel when the result is available
			//let on_complete = UnsafeSync::new(  );

			let data_ptr = Box::into_raw( Box::new(
				BrowserJsAsyncCallbackData::new( |_, result| {
					tx.send( result ).unwrap();
				} )
			) );

			unsafe { bw_BrowserWindow_evalJsAsync(
				handle.ffi_handle,
				js.into(),
				ffi_eval_js_async_callback,
				data_ptr as _
			) };
		}).await;


		rx.await.unwrap()
	}

	/// Causes the browser to navigate to the given url.
	///
	/// # Arguments
	/// * `url` - The url to navigate to
	pub async fn navigate( &self, url: &str ) -> Result<(), Box<dyn Error + Send>> {
		self.dispatch(|bw| {
			bw.navigate( url )
		}).await
	}
}

impl Deref for BrowserAsync {
	type Target = BrowserHandle;

	fn deref( &self ) -> &BrowserHandle {
		&self.handle
	}
}

impl Drop for BrowserAsync {
	fn drop( &mut self ) {
		unsafe { bw_Application_dispatch( self.app().handle.ffi_handle, ffi_free_browser_window, self.handle.ffi_handle as _ ); }
	}
}

impl From<BrowserHandle> for BrowserAsync {

	fn from( handle: BrowserHandle ) -> Self {
		Self {
			handle: handle
		}
	}
}

impl HasAppHandle for BrowserAsync {

	fn app_handle( &self ) -> ApplicationHandle {
		self.handle.app_handle()
	}
}



impl BrowserHandle {
	fn new( ffi_handle: *mut bw_BrowserWindow ) -> Self {
		Self {
			ffi_handle: ffi_handle
		}
	}
}

impl HasAppHandle for BrowserHandle {

	fn app_handle( &self ) -> ApplicationHandle {
		ApplicationHandle::new(
			unsafe { bw_BrowserWindow_getApp( self.ffi_handle ) }
		)
	}
}



extern "C" fn ffi_free_browser_window( _app: *mut bw_Application, data: *mut c_void ) {
	unsafe { bw_BrowserWindow_drop( data as *mut bw_BrowserWindow ); }
}

extern "C" fn ffi_eval_js_callback<H>( bw: *mut bw_BrowserWindow, cb_data: *mut c_void, result: *const c_char, error: *const bw_Err ) where
	H: FnOnce(Browser, Result<String, Box<dyn Error>>)
{

	let data_ptr = cb_data as *mut H;
	let data = unsafe { Box::from_raw( data_ptr ) };

	// Construct a result value depending on whether the result or error parameters are set
	let result_val: Result<String, Box<dyn Error>> = if error.is_null() {
		let result_str = unsafe { CStr::from_ptr( result ).to_string_lossy().to_owned().to_string() };
		Ok( result_str )
	}
	else {
		Err( Box::new( unsafe { (*error).clone() } ) )
	};

	let handle = Browser::from_ffi_handle( bw );

	(*data)( handle, result_val );
}

extern "C" fn ffi_eval_js_async_callback( bw: *mut bw_BrowserWindow, cb_data: *mut c_void, result: *const c_char, error: *const bw_Err ) {

	let data_ptr = cb_data as *mut BrowserJsAsyncCallbackData;
	let data = unsafe { Box::from_raw( data_ptr ) };

	// Construct a result value depending on whether the result or error parameters are set
	let result_val: Result<String, Box<dyn Error + Send>> = if error.is_null() {
		let result_str = unsafe { CStr::from_ptr( result ).to_string_lossy().to_owned().to_string() };
		Ok( result_str )
	}
	else {
		Err( Box::new( unsafe { (*error).clone() } ) )
	};

	let handle = BrowserHandle::new( bw );

	data.call( handle, result_val );
}
