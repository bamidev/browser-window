pub mod c;

pub use c::BrowserWindowImpl;
pub use c::JsEvaluationError;

use super::{
	application::ApplicationImpl,
	window::{WindowImpl, WindowOptions}
};

use browser_window_c::*;



pub type BrowserWindowOptions = cbw_BrowserWindowOptions;
pub type Source = cbw_BrowserWindowSource;

pub type CreationCallbackFn = unsafe fn( bw: BrowserWindowImpl, data: *mut () );
pub type EvalJsCallbackFn = unsafe fn( bw: BrowserWindowImpl, data: *mut (), result: Result<String, JsEvaluationError> ); 
pub type HandlerFn = unsafe fn( bw: BrowserWindowImpl, cmd: &str, args: Vec<String> );

pub trait BrowserWindowExt: Copy {

	/// Executes the given JavaScript string.
	/// The result will be provided by invoking the callback function.
	fn eval_js( &self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut () );
	
	/// Like `eval_js`, except it can be called from any thread.
	fn eval_js_threadsafe( &self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut () );

	/// Causes the browser to navigate to the given URI.
	fn navigate( &self, uri: &str );

	/// Creates a new browser window asynchronously.
	/// The `BrowserWindowImpl` handle to the new browser window will be passed via a callback.
	///
	/// # Arguments
	/// `app` - The application handle
	/// `parent` - An handle for another window that this window will be a child of. Use WindowImpl::default() for no parent.
	/// `source` - The content that will be displayed by the browser.
	/// `title` - The title that the window will have.
	/// `width` - The width of the window.
	/// `height` - The height of the window.
	/// `window_options` - Options for the window.
	/// `browser_window_options` - Some extra browser related options.
	/// `handler` - A handler function that can be invoked from within JavaScript code.
	/// `user_data` - Could be set to point to some extra data that this browser window will store.
	/// `creation_callback` - Will be invoked when the browser window is created. It provided the `BrowserWindowImpl` handle.
	/// `callback_data` - The data that will be provided to the `creation_callback`.
	fn new(
		app: ApplicationImpl,
		parent: WindowImpl,
		source: Source,
		title: &str,
		width: Option<u32>,
		height: Option<u32>,
		window_options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions,
		handler: &HandlerFn,
		user_data: *mut (),
		creation_callback: CreationCallbackFn,
		callback_data: *mut ()
	);

	fn user_data( &self ) -> *mut ();

	/// Gives a handle to the underlying window.
	fn window( &self ) -> WindowImpl;
}