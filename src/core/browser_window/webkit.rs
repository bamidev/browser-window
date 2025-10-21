use std::{
	borrow::Cow,
	cell::Cell,
	collections::HashMap,
	sync::atomic::{AtomicBool, Ordering},
};

use gtk::{gio::Cancellable, glib::CastNone, prelude::*};
use javascriptcore::ValueExt;
use webkit2gtk::{LoadEvent, Settings, SettingsExt, UserContentManagerExt, WebViewExt};

use super::{super::window::WindowImpl, *};
use crate::{
	def_browser_event, def_event,
	prelude::{ApplicationExt, WindowExt},
	rc::Rc,
};

#[derive(Clone)]
pub struct BrowserWindowImpl {
	inner: webkit2gtk::WebView,
}

struct EvalJsCallbackData {
	handle: BrowserWindowImpl,
	code: String,
	callback: EvalJsCallbackFn,
	data: *mut (),
}

/// An error that may occur when evaluating or executing JavaScript code.
pub type JsEvaluationError = webkit2gtk::Error;

impl BrowserWindowExt for BrowserWindowImpl {
	fn cookie_jar(&self) -> Option<CookieJarImpl> { None }

	fn eval_js(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ()) {
		let this = self.clone();
		self.inner
			.evaluate_javascript(js, None, None, Option::<&Cancellable>::None, move |r| {
				let result = match r {
					Ok(v) => Ok(transform_js_value(v)),
					// TODO: Test the error properly, not by testing message
					Err(e) =>
						if e.message() == "Unsupported result type" {
							Ok(JsValue::Undefined)
						} else {
							Err(e)
						},
				};
				callback(this, callback_data, result);
			});
	}

	fn eval_js_threadsafe(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ()) {
		let app = self.window().app();
		let dispatch_data = Box::new(EvalJsCallbackData {
			handle: self.clone(),
			code: js.to_owned(),
			callback,
			data: callback_data,
		});
		app.dispatch(dispatch_eval_js, Box::into_raw(dispatch_data) as _);
	}

	fn free(&self) {}

	fn navigate(&self, uri: &str) { self.inner.load_uri(uri); }

	fn new(
		app: ApplicationImpl, parent: WindowImpl, source: Source, title: &str, width: Option<u32>,
		height: Option<u32>, options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions, creation_callback: CreationCallbackFn,
		callback_data: *mut (),
	) {
		let window = WindowImpl::new(app, parent, title, width, height, options);
		let settings = Settings::builder().build();
		if browser_window_options.dev_tools > 0 {
			settings.set_enable_developer_extras(true);
		}
		let inner = webkit2gtk::WebView::builder().settings(&settings).build();
		let this = Self {
			inner: inner.clone(),
		};

		// Add the webview to the window
		window.0.add(&inner);

		// Load the source
		match source {
			Source::Url(url) => {
				inner.load_uri(&url);
			}
			Source::File(file) => {
				let uri = "file://".to_string()
					+ file
						.clone()
						.into_os_string()
						.into_string()
						.unwrap()
						.as_str();
				inner.load_uri(&uri);
			}
			Source::Html(html) => {
				inner.load_html(&html, None);
			}
		}

		// FIXME: We need to call creation_callback, but pass an error to it, if the web
		// view can not be loaded correctly.        Now we risk never notifying the
		// future that is waiting on us.
		let created = AtomicBool::new(false);
		inner.connect_load_changed(move |i, e| {
			if e == LoadEvent::Finished {
				// Create the global JS function `invoke_extern`
				i.evaluate_javascript(
					r#"
					function invoke_extern(...args) {
						window.webkit.messageHandlers.bw.postMessage([].slice.call(args))
					}
				"#,
					None,
					None,
					Option::<&Cancellable>::None,
					|r| {
						r.expect("invalid invoke_extern code");
					},
				);

				if !created.swap(true, Ordering::Relaxed) {
					creation_callback(this.clone(), callback_data);
				}
			}
		});
	}

	fn url(&self) -> Cow<'_, str> {
		self.inner
			.uri()
			.map(|g| g.to_string())
			.unwrap_or_default()
			.into()
	}

	fn window(&self) -> WindowImpl { WindowImpl(self.inner.toplevel().and_dynamic_cast().unwrap()) }
}

impl BrowserWindowEventExt for BrowserWindowImpl {
	fn on_message(&self, handle: Weak<BrowserWindowOwner>) -> MessageEvent {
		MessageEvent::new(handle)
	}
}

def_browser_event!(MessageEvent<MessageEventArgs>(&mut self, handler) {
	// Register a message handler
	let user_context_manager = self.owner.upgrade().unwrap().inner.inner.user_content_manager().unwrap();
	user_context_manager.register_script_message_handler("bw");
	let owner = self.owner.clone();
	let h = Rc::new(Cell::new(handler));
	user_context_manager.connect_script_message_received(Some("bw"), move |_, r| {
		if let Some(this) = owner.upgrade() {
			let value = r
				.js_value()
				.map(|v| transform_js_value(v))
				.unwrap_or(JsValue::Undefined);
			let (command, args) = match &value {
				JsValue::Array(a) => (a[0].to_string_unenclosed(), a[1..].to_vec()),
				_ => panic!("unexpected value type received from invoke_extern"),
			};

			let e = MessageEventArgs {
				cmd: command.to_string(),
				args
			};
			match unsafe { &mut *h.as_ptr() } {
				EventHandler::Sync(callback) => {
					(callback)(&*this, e);
				}
				EventHandler::Async(callback) => {
					let app = this.0.app();
					let future = (callback)(BrowserWindow(this.clone()), e);
					app.spawn(future);
				}
			}
		}
	});
});

fn transform_js_value(v: javascriptcore::Value) -> JsValue {
	if v.is_array() {
		let props = v.object_enumerate_properties();
		let mut vec = Vec::with_capacity(props.len());
		for i in 0..props.len() as u32 {
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

fn dispatch_eval_js(_app: ApplicationImpl, dispatch_data: *mut ()) {
	let data_ptr = dispatch_data as *mut EvalJsCallbackData;
	let data = unsafe { Box::from_raw(data_ptr) };

	let inner = data.handle.clone().inner;
	let callback = data.callback.clone();
	let handle = data.handle.clone();
	let callback_data = data.data.clone();
	inner.evaluate_javascript(
		&data.code,
		None,
		None,
		Option::<&Cancellable>::None,
		move |r| {
			let result = match r {
				Ok(v) => Ok(transform_js_value(v)),
				// TODO: Test the error properly, not by testing message
				Err(e) =>
					if e.message() == "Unsupported result type" {
						Ok(JsValue::Undefined)
					} else {
						Err(e)
					},
			};
			(callback)(handle, callback_data, result);
		},
	);
}
