use std::borrow::Cow;

use gtk::{gio::Cancellable, glib::{CastNone, IsA}, prelude::{ContainerExt, WidgetExt, WindowExtManual}};
use webkit2gtk::WebViewExt;

use crate::prelude::{ApplicationExt, WindowExt};

use super::{super::window::WindowImpl, *};

#[derive(Clone)]
pub struct BrowserWindowImpl(webkit2gtk::WebView);

struct CreationCallbackData {
	func: CreationCallbackFn,
	data: *mut (),
}

struct EvalJsCallbackData {
	callback: EvalJsCallbackFn,
	data: *mut (),
}

/// An error that may occur when evaluating or executing JavaScript code.
pub type JsEvaluationError = webkit2gtk::Error;

struct UserData {
	func: ExternalInvocationHandlerFn,
	data: *mut (),
}

impl BrowserWindowExt for BrowserWindowImpl {
	fn cookie_jar(&self) -> Option<CookieJarImpl> { None }

	fn eval_js(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ()) {
		let this = self.clone();
		self.0.call_async_javascript_function(js, None, None, None, Option::<&Cancellable>::None, move |r| {
			let result = r.map(|v| v.to_string());
			callback(this, callback_data, result);
		})
	}

	fn eval_js_threadsafe(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ()) {
		let app = self.window().app();
		//app.dispatch(dispatch_eval_js, callback_data);
		unimplemented!();
	}

	fn navigate(&self, uri: &str) { self.0.load_uri(uri); }

	fn new(
		app: ApplicationImpl, parent: WindowImpl, source: Source, title: &str, width: Option<u32>,
		height: Option<u32>, options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions, handler: ExternalInvocationHandlerFn,
		user_data: *mut (), creation_callback: CreationCallbackFn, callback_data: *mut (),
	) {
		let window = WindowImpl::new(app, parent, title, width, height, options, user_data);
		let inner = webkit2gtk::WebView::builder().build();
		window.0.add(&inner);

		creation_callback(Self(inner), callback_data)
	}

	fn user_data(&self) -> *mut () {
		unsafe { *self.0.window().unwrap().user_data() }
	}

	fn url(&self) -> Cow<'_, str> { self.0.uri().map(|g| g.to_string()).unwrap_or_default().into() }

	fn window(&self) -> WindowImpl {
		let inner: gtk::Window = self.0.toplevel().and_dynamic_cast().unwrap();
		WindowImpl (inner)
	}
}
