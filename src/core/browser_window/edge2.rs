use std::borrow::Cow;

use webview2::Environment;
use winapi::{shared::windef, um::winuser};

use super::{super::window::WindowImpl, *};
use crate::prelude::{ApplicationExt, WindowExt};


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
			Box::<UserData>::from_raw((*self.inner).user_data as _);
			Box::<webview2::Controller>::from_raw((*self.inner).impl_.controller as _);
			Box::<webview2::WebView>::from_raw((*self.inner).impl_.webview as _);
		}
	}

	fn navigate(&self, uri: &str) { self.webview().navigate(uri); }

	fn new(
		app: ApplicationImpl, parent: WindowImpl, source: Source, title: &str, width: Option<u32>,
		height: Option<u32>, window_options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions, creation_callback: CreationCallbackFn, callback_data: *mut (),
	) {
		// Create window
		let user_data = Box::new(UserData {
			func: handler,
			data: user_data,
		});
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

					webview.add_navigation_completed(move |wv, e| {
						let handle = BrowserWindowImpl { inner: bw_inner };
						creation_callback(handle, callback_data);
						Ok(())
					});

					Ok(())
				});
				Ok(())
			})
			.unwrap();
	}

	fn user_data(&self) -> *mut () {
		let c_user_data_ptr: *mut UserData = unsafe { (*self.inner).user_data as _ };

		// The actual user data pointer is stored within the `UserData` struct that is
		// stored within the C handle
		unsafe { (*c_user_data_ptr).data }
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
	fn on_message(&self, handle: Weak<BrowserWindowOwner>) -> MessageEvent { MessageEvent::new(handle) }
}

def_browser_event!(MessageEvent<MessageEventArgs<'static>>(&mut self, handler) {
	if let Some(this) = self.owner.upgrade() {
		// Register the message handler
		let this2 = self.clone();
		webview.add_web_message_received(move |w, msg| {
			let string = msg
				.get_web_message_as_json()
				.expect("unable to get web message as json");

			let handle = BrowserWindowImpl { inner: bw_inner };
			let(command, args) = match JsValue::from_json(&string) {
				JsValue::Array(args) => {
					let command = args[0].to_string_unenclosed();
					let command_args = args[1..].to_vec();
					(command, command_args)
				}
				_ => panic!(
					"unexpected JavaScript value received from Edge WebView2"
				),
			}

			let e = MessageEventArgs {
				cmd: unsafe { &*(command.as_ref() as *const str) },
				args
			};
			match unsafe { &mut *h.as_ptr() } {
				EventHandler::Sync(callback) => {
					(callback)(&*this2, &e);
				}
				EventHandler::Async(callback) => {
					let app = this2.0.app();
					let future = (callback)(BrowserWindow(this2.clone()), &e);
					app.spawn(future);
				}
			}
			Ok(())
		})
		.expect("unable to register message handler");
	}
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

/*unsafe extern "C" fn ffi_handler(
	bw: *mut cbw_BrowserWindow, cmd: cbw_CStrSlice, args: *mut cbw_CStrSlice, arg_count: usize,
) {
	let handle = BrowserWindowImpl { inner: bw };

	let data_ptr = (*bw).user_data as *mut UserData;
	let data = &mut *data_ptr;

	// Convert the command and args to a String and `Vec<&str>`
	let cmd_string: &str = cmd.into();
	let mut args_vec: Vec<JsValue> = Vec::with_capacity(arg_count as usize);
	for i in 0..arg_count {
		args_vec.push(JsValue::Other((*args.add(i as usize)).into()));
	}

	(data.func)(handle, cmd_string, args_vec);
}*/
