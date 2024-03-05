use std::{error::Error as StdError, ffi::CStr, fmt, mem::MaybeUninit, os::raw::*, ptr};
use std::{slice, str};

use browser_window_c::*;

use super::{super::{error::Error, window::WindowImpl}, *};

use crate::{def_browser_event, def_event, event::EventHandlerCallback};


#[derive(Clone)]
pub struct BrowserWindowImpl {
	inner: *mut cbw_BrowserWindow,
}

struct CreationCallbackData {
	func: CreationCallbackFn,
	data: *mut (),
}

struct EvalJsCallbackData {
	callback: EvalJsCallbackFn,
	data: *mut (),
}

/// An error that may occur when evaluating or executing JavaScript code.
#[derive(Debug)]
pub struct JsEvaluationError {
	message: String, /* TODO: Add line and column number files, and perhaps even more info about
	                  * the JS error */
}

struct UserData {
	func: ExternalInvocationHandlerFn,
	data: *mut (),
}

struct EventData<'a, C, A> {
	handle: BrowserWindowHandle,
	handler: BrowserWindowEventHandler<'a, A>,
	converter: unsafe fn(&C) -> A
}


#[doc(hidden)]
#[macro_export]
macro_rules! def_browser_event_c {
	($name:ident<$carg_type:ty, $rarg_type:ty> => $converter:ident => $c_event_name:ident) => {
		def_browser_event!($name<$rarg_type>(&mut self, handler) {
			let c_ptr = unsafe { &mut *self.handle.inner.inner };
			// Free the previous event data if overwriting
			if c_ptr.events.$c_event_name.callback.is_some() {
				unsafe { let _ = Box::from_raw(c_ptr.events.$c_event_name.data as *mut EventData<'static, $carg_type, $rarg_type>); }
			}
			
			// Store the new event data
			let event_data = EventData::<$carg_type, $rarg_type> {
				handle: unsafe { self.handle.clone() },
				handler,
				converter: $converter,
			};
			let event_data_ptr = Box::into_raw(Box::new(event_data));
			c_ptr.events.$c_event_name = cbw_Event {
				callback: Some(ffi_browser_window_event_callback::<$carg_type, $rarg_type>),
				data: event_data_ptr as _
			};
		});
	}
}


impl BrowserWindowExt for BrowserWindowImpl {
	fn cookie_jar(&self) -> Option<CookieJarImpl> {
		let inner = unsafe { cbw_CookieJar_newGlobal() };

		Some(CookieJarImpl { inner })
	}

	fn eval_js(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ()) {
		let data = Box::new(EvalJsCallbackData {
			callback,
			data: callback_data,
		});
		let data_ptr = Box::into_raw(data);

		unsafe {
			cbw_BrowserWindow_evalJs(
				self.inner,
				js.into(),
				Some(ffi_eval_js_callback_handler),
				data_ptr as _,
			)
		}
	}

	fn eval_js_threadsafe(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ()) {
		let data = Box::new(EvalJsCallbackData {
			callback,
			data: callback_data,
		});

		let data_ptr = Box::into_raw(data);

		unsafe {
			cbw_BrowserWindow_evalJsThreaded(
				self.inner,
				js.into(),
				Some(ffi_eval_js_callback_handler),
				data_ptr as _,
			)
		}
	}

	fn free(&self) {
		unsafe {
			let _ = Box::<UserData>::from_raw((*self.inner).user_data as _);
		}
	}

	fn navigate(&self, uri: &str) { unsafe { cbw_BrowserWindow_navigate(self.inner, uri.into()) }; }

	fn user_data(&self) -> *mut () {
		let c_user_data_ptr: *mut UserData = unsafe { (*self.inner).user_data as _ };

		// The actual user data pointer is stored within the `UserData` struct that is
		// stored within the C handle
		unsafe { (*c_user_data_ptr).data }
	}

	fn url<'a>(&'a self) -> Cow<'a, str> {
		let mut slice: cbw_StrSlice = unsafe { MaybeUninit::uninit().assume_init() };
		let owned = unsafe { cbw_BrowserWindow_getUrl(self.inner, &mut slice) };

		if owned > 0 {
			let url: String = slice.into();
			unsafe { cbw_string_free(slice) };
			url.into()
		} else {
			let url: &'a str = slice.into();
			url.into()
		}
	}

	fn window(&self) -> WindowImpl {
		WindowImpl {
			inner: unsafe { cbw_BrowserWindow_getWindow(self.inner) },
		}
	}
}

impl BrowserWindowImpl {
	pub(crate) fn new(
		app: ApplicationImpl, parent: WindowImpl, source: Source, title: &str, width: Option<u32>,
		height: Option<u32>, window_options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions, handler: ExternalInvocationHandlerFn,
		_user_data: *mut (), creation_callback: CreationCallbackFn, _callback_data: *mut (),
	) {
		// Convert width and height to -1 if unspecified.
		// Also convert to c_int as required by the C interface.
		let w: c_int = match width {
			None => -1,
			Some(x) => x as _,
		};
		let h: c_int = match height {
			None => -1,
			Some(x) => x as _,
		};

		// Wrap the callback functions so that they invoke our Rust functions from C
		let user_data = Box::new(UserData {
			func: handler,
			data: _user_data,
		});
		let callback_data = Box::new(CreationCallbackData {
			func: creation_callback,
			data: _callback_data,
		});

		// Source
		let mut _url: String = "file:///".into(); // Stays here so that the reference to it that gets passed to C stays valid for the function call to `bw_BrowserWindow_new`.
		let source2 = match &source {
			// Use a reference, we want source to live until the end of the function because
			// bw_BrowserWindowSource holds a reference to its internal string.
			Source::File(path) => {
				_url += path.to_str().unwrap();

				cbw_BrowserWindowSource {
					data: _url.as_str().into(),
					is_html: 0,
				}
			}
			Source::Html(html) => cbw_BrowserWindowSource {
				data: html.as_str().into(),
				is_html: 1,
			},
			Source::Url(url) => cbw_BrowserWindowSource {
				data: url.as_str().into(),
				is_html: 0,
			},
		};

		unsafe {
			let browser = cbw_BrowserWindow_new(
				app.inner,
				parent.inner,
				title.into(),
				w,
				h,
				window_options as _,
				Some(ffi_handler),
				// FIXME: user_data is leaked into memory.
				Box::into_raw(user_data) as _,
			);
			cbw_BrowserWindow_create(
				browser,
				w,
				h,
				source2,
				browser_window_options as _,
				Some(ffi_creation_callback_handler),
				Box::into_raw(callback_data) as _,
			);
		};
	}
}

impl BrowserWindowEventExt for BrowserWindowImpl {
	fn on_page_title_changed<'a>(&self, handle: &'a BrowserWindowHandle) -> PageTitleChangedEvent<'a> {println!("register on_page_title_changed");  PageTitleChangedEvent::new(handle) }
	fn on_navigation_end<'a>(&self, handle: &'a BrowserWindowHandle) -> NavigationEndEvent<'a> { NavigationEndEvent::new(handle) }
	fn on_navigation_start<'a>(&self, handle: &'a BrowserWindowHandle) -> NavigationStartEvent<'a> { NavigationStartEvent::new(handle) }
	fn on_tooltip<'a>(&self, handle: &'a BrowserWindowHandle) -> TooltipEvent<'a> { TooltipEvent::new(handle) }
}

def_browser_event_c!(NavigationStartEvent<(), ()> => no_converter => on_navigation_start);
def_browser_event_c!(NavigationEndEvent<cbw_Err, Error> => error_converter => on_navigation_end);
def_browser_event_c!(PageTitleChangedEvent<cbw_CStrSlice, &str> => str_converter => on_page_title_changed);
def_browser_event_c!(TooltipEvent<cbw_StrSlice, &mut str> => str_mut_converter => on_tooltip);

impl JsEvaluationError {
	pub(super) unsafe fn new(err: *const cbw_Err) -> Self {
		let msg_ptr = ((*err).alloc_message.unwrap())((*err).code, (*err).data);
		let cstr = CStr::from_ptr(msg_ptr);
		let message: String = cstr.to_string_lossy().into();

		Self { message }
	}
}

impl StdError for JsEvaluationError {
	fn source(&self) -> Option<&(dyn StdError + 'static)> { None }
}

impl fmt::Display for JsEvaluationError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.message.as_str())
	}
}

/***************************************************************************
 * ************************************* The C handler functions that are
 * invoked by external C code, and that in turn invoke relevant Rust
 * handlers. * *************************************************************
 * ************************************************ */

pub(super) unsafe extern "C" fn ffi_creation_callback_handler(
	bw: *mut cbw_BrowserWindow, _data: *mut c_void,
) {
	let data_ptr = _data as *mut CreationCallbackData;
	let data = Box::from_raw(data_ptr);

	let handle = BrowserWindowImpl { inner: bw };

	(data.func)(handle, data.data);
}

unsafe extern "C" fn ffi_eval_js_callback_handler(
	bw: *mut cbw_BrowserWindow, _data: *mut c_void, _result: *const c_char, error: *const cbw_Err,
) {
	let data_ptr = _data as *mut EvalJsCallbackData;
	let data = Box::from_raw(data_ptr);

	let (handle, result) = ffi_eval_js_callback_result(bw, _result, error);

	(data.callback)(handle, data.data, result);
}

unsafe extern "C" fn ffi_handler(
	bw: *mut cbw_BrowserWindow, cmd: cbw_CStrSlice, args: *mut cbw_CStrSlice, arg_count: usize,
) {
	let handle = BrowserWindowImpl { inner: bw };

	let data_ptr = (*bw).user_data as *mut UserData;
	let data = &mut *data_ptr;

	// Convert the command and args to a String and `Vec<&str>`
	let cmd_string: &str = cmd.into();
	let mut args_vec: Vec<JsValue> = Vec::with_capacity(arg_count as usize);
	for i in 0..arg_count {
		args_vec.push(JsValue::from_string((*args.add(i as usize)).into()));
	}

	(data.func)(handle, cmd_string, args_vec);
}

/// Processes the result received from the C function, and returns it in a Rust
/// Result.
unsafe fn ffi_eval_js_callback_result(
	bw: *mut cbw_BrowserWindow, result: *const c_char, error: *const cbw_Err,
) -> (BrowserWindowImpl, Result<JsValue, JsEvaluationError>) {
	// Construct a result value depending on whether the result or error parameters
	// are set
	let result_val: Result<JsValue, JsEvaluationError> = if error.is_null() {
		let result_str = CStr::from_ptr(result).to_string_lossy().to_string();

		// Parse the string
		Ok(JsValue::from_string(&result_str))
	} else {
		Err(JsEvaluationError::new(error))
	};

	let handle = BrowserWindowImpl { inner: bw };

	// return
	(handle, result_val)
}

unsafe extern "C" fn ffi_browser_window_event_callback<C, A>(handler_data: *mut c_void, arg_ptr: *mut c_void) -> i32 { println!("ffi_browser_window_event_callback");
	let event_data_ptr = handler_data as *mut EventData<'static, C, A>;
	let event_data = &mut *event_data_ptr;
	let arg_ptr2 = arg_ptr as *mut C;
	let carg = &*arg_ptr2;

	// Convert C type to Rust type
	let rarg = (event_data.converter)(carg);

	(event_data.handler)(&event_data.handle, &rarg);
	return 0;
}

unsafe fn no_converter(input: &()) -> () { () }

unsafe fn error_converter(input: &cbw_Err) -> Error {
	let err2 = input.clone();
	Error::from(err2)
}

unsafe fn str_converter(input: &cbw_CStrSlice) -> &'static str {
	str::from_utf8_unchecked(slice::from_raw_parts(input.data as *const u8, input.len) as _)
}

unsafe fn str_mut_converter(input: &cbw_StrSlice) -> &'static mut str {
	str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(input.data as *mut u8, input.len) as _)
}
