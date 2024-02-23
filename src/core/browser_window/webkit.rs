use std::{borrow::Cow, collections::HashMap};

use gtk::{
	gio::Cancellable,
	glib::{CastNone, IsA},
	prelude::{ContainerExt, WidgetExt, WindowExtManual},
};
use javascriptcore::{Value, ValueExt};
use webkit2gtk::WebViewExt;

use super::{super::window::WindowImpl, *};
use crate::prelude::{ApplicationExt, WindowExt};

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
		self.0
			.evaluate_javascript(js, None, None, Option::<&Cancellable>::None, move |r| {
				let result = r.map(|v| transform_js_value(v));
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

	fn user_data(&self) -> *mut () { unsafe { *self.0.window().unwrap().user_data() } }

	fn url(&self) -> Cow<'_, str> {
		self.0
			.uri()
			.map(|g| g.to_string())
			.unwrap_or_default()
			.into()
	}

	fn window(&self) -> WindowImpl {
		let inner: gtk::Window = self.0.toplevel().and_dynamic_cast().unwrap();
		WindowImpl(inner)
	}
}

fn transform_js_value(v: javascriptcore::Value) -> JsValue {
	if v.is_array() {
		let mut vec = Vec::with_capacity(v.array_buffer_get_size());
		for i in 0..vec.capacity() as u32 {
			let iv = v.object_get_property_at_index(i).unwrap();
			vec.push(transform_js_value(iv));
		}
		JsValue::Array(vec)
	} else if v.is_boolean() {
		JsValue::Boolean(v.to_boolean())
	} else if v.is_null() {
		JsValue::Null
	} else if v.is_number() {
		JsValue::Number(v.to_double().into())
	} else if v.is_object() {
		let props = v.object_enumerate_properties();
		let mut map = HashMap::with_capacity(props.len());
		for prop in props {
			let pv = v.object_get_property(&prop).unwrap();
			map.insert(prop.to_string(), transform_js_value(pv));
		}
		JsValue::Object(map)
	} else if v.is_string() {
		JsValue::String(v.to_str().into())
	} else if v.is_undefined() {
		JsValue::Undefined
	} else {
		JsValue::Other(v.to_str().to_string())
	}
}
