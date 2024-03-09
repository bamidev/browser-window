use std::{
	error::Error as StdError, ffi::CStr, fmt, mem::MaybeUninit, os::raw::*, ptr, slice, str,
};

use browser_window_c::*;

use super::{
	super::{error::Error, window::WindowImpl},
	*,
};
use crate::{
	def_browser_event, def_event,
	rc::{Rc, Weak},
};


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

struct BrowserWindowUserData {
	_owner: Rc<BrowserWindowOwner>,
}

struct EventData<C, A> {
	owner: Weak<BrowserWindowOwner>,
	handler: BrowserWindowEventHandler<A>,
	converter: unsafe fn(&C) -> A,
}


#[doc(hidden)]
#[macro_export]
macro_rules! def_browser_event_c {
	($name:ident<$carg_type:ty, $rarg_type:ty> => $converter:ident => $c_event_name:ident) => {
		def_browser_event!($name<$rarg_type>(&mut self, handler) {
			if let Some(upgraded) = self.owner.upgrade() {
				let c_ptr = unsafe { &mut *upgraded.0.inner.inner };
				// Free the previous event data if overwriting
				if c_ptr.events.$c_event_name.callback.is_some() {
					unsafe { let _ = Box::from_raw(c_ptr.events.$c_event_name.data as *mut EventData<$carg_type, $rarg_type>); }
				}

				// Store the new event data
				let event_data = EventData::<$carg_type, $rarg_type> {
					owner: self.owner.clone(),
					handler,
					converter: $converter,
				};
				let event_data_ptr = Box::into_raw(Box::new(event_data));
				c_ptr.events.$c_event_name = cbw_Event {
					callback: Some(ffi_browser_window_event_callback::<$carg_type, $rarg_type>),
					data: event_data_ptr as _
				};
			}
		});
	}
}


impl BrowserWindowExt for BrowserWindowImpl {
	fn cookie_jar(&self) -> Option<CookieJarImpl> {
		let inner = unsafe { cbw_CookieJar_newGlobal() };

		Some(CookieJarImpl(inner))
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

	fn free(&self) { unsafe { cbw_BrowserWindow_free(self.inner) } }

	fn navigate(&self, uri: &str) { unsafe { cbw_BrowserWindow_navigate(self.inner, uri.into()) }; }

	fn url<'a>(&'a self) -> Cow<'a, str> {
		let owned;
		let slice;
		unsafe {
			let mut slice_uninit: MaybeUninit<cbw_StrSlice> = MaybeUninit::uninit();
			owned = cbw_BrowserWindow_getUrl(self.inner, slice_uninit.as_mut_ptr());
			slice = slice_uninit.assume_init();
		}

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

	fn new(
		app: ApplicationImpl, parent: WindowImpl, source: Source, title: &str, width: Option<u32>,
		height: Option<u32>, window_options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions, creation_callback: CreationCallbackFn,
		_callback_data: *mut (),
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
	fn on_address_changed(&self, handle: Weak<BrowserWindowOwner>) -> AddressChangedEvent {
		AddressChangedEvent::new(handle)
	}

	fn on_console_message(&self, handle: Weak<BrowserWindowOwner>) -> ConsoleMessageEvent {
		ConsoleMessageEvent::new(handle)
	}

	fn on_favicon_changed(&self, handle: Weak<BrowserWindowOwner>) -> FaviconChangedEvent {
		FaviconChangedEvent::new(handle)
	}

	fn on_fullscreen_mode_changed(
		&self, handle: Weak<BrowserWindowOwner>,
	) -> FullscreenModeChangedEvent {
		FullscreenModeChangedEvent::new(handle)
	}

	fn on_loading_progress_changed(
		&self, handle: Weak<BrowserWindowOwner>,
	) -> LoadingProgressChangedEvent {
		LoadingProgressChangedEvent::new(handle)
	}

	fn on_message(&self, handle: Weak<BrowserWindowOwner>) -> MessageEvent {
		MessageEvent::new(handle)
	}

	fn on_navigation_end(&self, handle: Weak<BrowserWindowOwner>) -> NavigationEndEvent {
		NavigationEndEvent::new(handle)
	}

	fn on_navigation_start(&self, handle: Weak<BrowserWindowOwner>) -> NavigationStartEvent {
		NavigationStartEvent::new(handle)
	}

	fn on_page_title_changed(&self, handle: Weak<BrowserWindowOwner>) -> PageTitleChangedEvent {
		PageTitleChangedEvent::new(handle)
	}

	fn on_status_message(&self, handle: Weak<BrowserWindowOwner>) -> StatusMessageEvent {
		StatusMessageEvent::new(handle)
	}

	fn on_tooltip(&self, handle: Weak<BrowserWindowOwner>) -> TooltipEvent {
		TooltipEvent::new(handle)
	}
}

def_browser_event_c!(AddressChangedEvent<cbw_CStrSlice, &str> => str_converter => on_address_changed);
def_browser_event_c!(ConsoleMessageEvent<cbw_CStrSlice, &str> => str_converter => on_console_message);
def_browser_event_c!(FaviconChangedEvent<cbw_CStrSlice, &str> => str_converter => on_favicon_changed);
def_browser_event_c!(FullscreenModeChangedEvent<c_int, bool> => bool_converter => on_fullscreen_mode_changed);
def_browser_event_c!(LoadingProgressChangedEvent<c_double, f64> => f64_converter => on_loading_progress_changed);
def_browser_event_c!(MessageEvent<cbw_BrowserWindowMessageArgs, MessageEventArgs<'static>> => message_args_converter => on_message);
def_browser_event_c!(NavigationStartEvent<(), ()> => no_converter => on_navigation_start);
def_browser_event_c!(NavigationEndEvent<cbw_Err, Result<(), Error>> => error_converter => on_navigation_end);
def_browser_event_c!(PageTitleChangedEvent<cbw_CStrSlice, &str> => str_converter => on_page_title_changed);
def_browser_event_c!(StatusMessageEvent<cbw_CStrSlice, &str> => str_converter => on_status_message);
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

unsafe extern "C" fn ffi_browser_window_event_callback<C, A>(
	handler_data: *mut c_void, arg_ptr: *mut c_void,
) -> i32 {
	let event_data_ptr = handler_data as *mut EventData<C, A>;
	let event_data = &mut *event_data_ptr;
	let arg_ptr2 = arg_ptr as *mut C;
	let carg = &*arg_ptr2;

	// Convert C type to Rust type
	let rarg = (event_data.converter)(carg);

	// Run the event handler
	let rc_handle = event_data
		.owner
		.upgrade()
		.expect("browser window handle is gone");
	match &mut event_data.handler {
		EventHandler::Sync(callback) => {
			(callback)(&*rc_handle, &rarg);
		}
		EventHandler::Async(callback) => {
			let app = rc_handle.0.app();
			let future = (callback)(BrowserWindow(rc_handle.clone()), &rarg);
			app.spawn(future);
		}
	}
	return 0;
}

unsafe fn no_converter(_input: &()) -> () { () }

unsafe fn error_converter(input: &cbw_Err) -> Result<(), Error> {
	if input.code == 0 {
		Ok(())
	} else {
		let err2 = input.clone();
		Err(Error::from(err2))
	}
}

unsafe fn bool_converter(input: &c_int) -> bool { *input > 0 }

unsafe fn f64_converter(input: &c_double) -> f64 { *input }

unsafe fn str_converter(input: &cbw_CStrSlice) -> &'static str {
	str::from_utf8_unchecked(slice::from_raw_parts(input.data as *const u8, input.len) as _)
}

unsafe fn str_mut_converter(input: &cbw_StrSlice) -> &'static mut str {
	str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(input.data as *mut u8, input.len) as _)
}

unsafe fn message_args_converter(
	input: &cbw_BrowserWindowMessageArgs,
) -> MessageEventArgs<'static> {
	// Convert the command and args to a String and `Vec<&str>`
	let cmd_string = str::from_utf8_unchecked(slice::from_raw_parts(
		input.cmd.data as *const u8,
		input.cmd.len,
	));
	let mut args_vec: Vec<JsValue> = Vec::with_capacity(input.arg_count as usize);
	for i in 0..input.arg_count {
		args_vec.push(JsValue::from_string((*input.args.add(i as usize)).into()));
	}

	MessageEventArgs {
		cmd: cmd_string,
		args: args_vec,
	}
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
