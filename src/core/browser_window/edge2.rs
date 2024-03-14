use std::{borrow::Cow, cell::Cell, ffi::c_void, ptr};

use webview2::Environment;
use winapi::{shared::windef, um::winuser};

use super::{super::window::WindowImpl, *};
use crate::{
	def_browser_event, def_event,
	prelude::{ApplicationExt, WindowExt},
};


#[derive(Clone)]
pub struct BrowserWindowImpl {
	inner: *mut cbw_BrowserWindow,
}

struct EvalJsCallbackData {
	handle: BrowserWindowImpl,
	code: String,
	callback: EvalJsCallbackFn,
	data: *mut (),
}

/// An error that may occur when evaluating or executing JavaScript code.
pub type JsEvaluationError = ();


impl BrowserWindowImpl {
	fn controller(&self) -> &webview2::Controller {
		unsafe {
			let ptr = (*self.inner).impl_.controller as *const webview2::Controller;
			&*ptr
		}
	}

	fn webview(&self) -> &webview2::WebView {
		unsafe {
			let ptr = (*self.inner).impl_.webview as *const webview2::WebView;
			&*ptr
		}
	}
}

impl BrowserWindowExt for BrowserWindowImpl {
	fn cookie_jar(&self) -> Option<CookieJarImpl> { None }

	fn eval_js(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ()) {
		let this = self.clone();
		self.webview().execute_script(js, move |result| {
			callback(this, callback_data, Ok(JsValue::from_json(&result)));
			Ok(())
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

	fn free(&self) {
		unsafe {
			Box::<webview2::Controller>::from_raw((*self.inner).impl_.controller as _);
			Box::<webview2::WebView>::from_raw((*self.inner).impl_.webview as _);
		}
	}

	fn navigate(&self, uri: &str) { self.webview().navigate(uri); }

	fn new(
		app: ApplicationImpl, parent: WindowImpl, source: Source, title: &str, width: Option<u32>,
		height: Option<u32>, window_options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions, creation_callback: CreationCallbackFn,
		callback_data: *mut (),
	) {
		// Create window
		let bw_inner = unsafe {
			cbw_BrowserWindow_new(
				app.inner,
				parent.inner,
				title.into(),
				width.unwrap_or(0) as _,
				height.unwrap_or(0) as _,
				window_options as _,
			)
		};

		let hwnd = unsafe { (*(*bw_inner).window).impl_.handle as windef::HWND };
		let options = browser_window_options.clone();
		Environment::builder()
			.build(move |renv| {
				let env = renv.expect("environment error");
				env.create_controller(hwnd, move |rcon| {
					let controller = Box::new(rcon.expect("controller error"));
					// This line is necessary because the webview doesn't show otherwise, if the
					// window hasn't been shown yet.
					controller.put_is_visible(true);
					let webview =
						Box::new(controller.get_webview().expect("unable to get webview"));
					unsafe {
						(*bw_inner).impl_.controller = Box::into_raw(controller.clone()) as _;
						(*bw_inner).impl_.webview = Box::into_raw(webview.clone()) as _;
					}

					let settings = webview.get_settings().expect("unable to get settings");
					settings.put_is_script_enabled(true);
					settings.put_are_default_script_dialogs_enabled(true);
					settings.put_is_web_message_enabled(true);
					settings.put_are_dev_tools_enabled(options.dev_tools > 0);

					let mut rect = windef::RECT {
						left: 0,
						top: 0,
						right: 0,
						bottom: 0,
					};
					unsafe { winuser::GetClientRect(hwnd, &mut rect) };
					controller.put_bounds(rect);

					let result = match source {
						Source::Url(url) => webview.navigate(&url),
						Source::Html(content) => webview.navigate_to_string(&content),
						Source::File(path) => {
							let file_uri = path.to_string_lossy();
							webview.navigate(&file_uri)
						}
					};
					result.expect("unable to navigate to source");

					webview.execute_script(
						r#"
						function invoke_extern(...args) {
							window.chrome.webview.postMessage([].slice.call(args));
						}
					"#,
						move |_| Ok(()),
					);

					let mut created = AtomicBool::new(false);;
					webview.add_navigation_completed(move |wv, e| {
						if !created.swap(true, Ordering::Relaxed) {
							let handle = BrowserWindowImpl { inner: bw_inner };
							creation_callback(handle, callback_data);
						}
						Ok(())
					});

					Ok(())
				});
				Ok(())
			})
			.unwrap();
	}

	fn url(&self) -> Cow<'_, str> {
		self.webview()
			.get_source()
			.expect("unable to get source")
			.into()
	}

	fn window(&self) -> WindowImpl {
		unsafe {
			WindowImpl {
				inner: (*self.inner).window,
			}
		}
	}
}

impl BrowserWindowEventExt for BrowserWindowImpl {
	fn on_message(&self, handle: Weak<BrowserWindowOwner>) -> MessageEvent {
		MessageEvent::new(handle)
	}
}


def_browser_event!(MessageEvent<MessageEventArgs>(&mut self, handler) {

	// Register the message handler
	let owner = self.owner.clone();
	let h = Rc::new(Cell::new(handler));
	let inner = &owner.upgrade().unwrap().inner;
	inner.webview().add_web_message_received(move |_, msg| {
		if let Some(this) = owner.upgrade() {
			let string = msg
				.get_web_message_as_json()
				.expect("unable to get web message as json");

			let(command, args2) = match JsValue::from_json(&string) {
				JsValue::Array(args) => {
					let command = args[0].to_string_unenclosed().to_string();
					let command_args = args[1..].to_vec();
					(command, command_args)
				}
				_ => panic!(
					"unexpected JavaScript value received from Edge WebView2"
				),
			};

			let e = MessageEventArgs {
				cmd: command,
				args: args2
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
		Ok(())
	})
	.expect("unable to register message handler");
});


fn dispatch_eval_js(_app: ApplicationImpl, dispatch_data: *mut ()) {
	let data_ptr = dispatch_data as *mut EvalJsCallbackData;
	let data = unsafe { Box::from_raw(data_ptr) };

	let callback = data.callback.clone();
	let handle = data.handle.clone();
	let callback_data = data.data.clone();
	handle
		.clone()
		.webview()
		.execute_script(&data.code, move |result| {
			callback(handle, callback_data, Ok(JsValue::from_json(&result)));
			Ok(())
		});
}


#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn bw_Window_freeUserData(w: *mut c_void) {
	let w_ptr = w as *mut cbw_Window;
	unsafe {
		if (*w_ptr).user_data != ptr::null_mut() {
			BrowserWindowImpl::free_user_data((*w_ptr).user_data as _);
			(*w_ptr).user_data = ptr::null_mut();
		}
	}
}
