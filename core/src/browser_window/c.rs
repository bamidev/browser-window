use super::*;

use std::{
	error::Error,
	ffi::CStr,
	fmt,
	mem,
	os::raw::*
};

use browser_window_c::*;

use crate::window::WindowImpl;



#[derive(Clone,Copy)]
pub struct BrowserWindowImpl {
	inner: *mut cbw_BrowserWindow
}

struct CreationCallbackData {
	func: CreationCallbackFn,
	data: *mut ()
}

struct EvalJsCallbackData {
	callback: EvalJsCallbackFn,
	data: *mut ()
}

#[allow(dead_code)]
struct UserData {
	func: *const HandlerFn,
	data: *mut ()
}

/// An error that may occur when evaluating or executing JavaScript code.
#[derive(Debug)]
pub struct JsEvaluationError {
	message: String
	// TODO: Add line and column number files, and perhaps even more info about the JS error
}



impl BrowserWindowExt for BrowserWindowImpl {

	fn eval_js( &self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut () ) {
		let data = Box::new( EvalJsCallbackData {
			callback,
			data: callback_data
		} );

		let data_ptr = Box::into_raw( data );

		unsafe { cbw_BrowserWindow_evalJs( self.inner, js.into(), Some( ffi_eval_js_callback_handler ), data_ptr as _ ) }
	}

	fn eval_js_threadsafe( &self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut () ) {
		let data = Box::new( EvalJsCallbackData {
			callback,
			data: callback_data
		} );

		let data_ptr = Box::into_raw( data );

		unsafe { cbw_BrowserWindow_evalJsThreaded( self.inner, js.into(), Some( ffi_eval_js_callback_handler ), data_ptr as _ ) }
	}

	fn navigate( &self, uri: &str ) {
		unsafe { cbw_BrowserWindow_navigate( self.inner, uri.into() ) };
	}

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
		_user_data: *mut (),
		creation_callback: CreationCallbackFn,
		_callback_data: *mut ()
	) {
		// Convert width and height to -1 if unspecified.
		// Also convert to c_int as required by the C interface.
		let w: c_int = match width {
			None => -1,
			Some(x) => x as _
		};
		let h: c_int = match height {
			None => -1,
			Some(x) => x as _
		};

		// Wrap the callback functions so that they invoke our Rust functions from C
		let user_data = Box::new( UserData {
			func: handler,
			data: _user_data
		} );
		let callback_data = Box::new( CreationCallbackData {
			func: creation_callback,
			data: _callback_data
		} );

		unsafe { cbw_BrowserWindow_new(
			app.inner,
			parent.inner,
			source,
			title.into(),
			w, h,
			window_options as _,
			browser_window_options as _,
			// Apparently C's size_t doesn't translate to usize.
			// However, if I'd use a u64 as the functions arg type, we'd run into problems on 32-bit systems.
			// So I basically force the cast right here.
			Some( mem::transmute( &ffi_handler ) ),
			Box::into_raw( user_data ) as _,
			Some( ffi_creation_callback_handler ),
			Box::into_raw( callback_data ) as _
		) };
	}

	fn user_data( &self ) -> *mut () {
		unsafe { (*self.inner).user_data as _ }
	}

	fn window( &self ) -> WindowImpl {
		WindowImpl {
			inner: unsafe { cbw_BrowserWindow_getWindow( self.inner ) }
		}
	}
}



impl JsEvaluationError {
	pub(in super) unsafe fn new( err: *const cbw_Err ) -> Self {

		let msg_ptr = ((*err).alloc_message.unwrap())( (*err).code, (*err).data );
		let cstr = CStr::from_ptr( msg_ptr );
		let message: String = cstr.to_string_lossy().into();

		Self {
			message: message
		}
	}
}

impl Error for JsEvaluationError {
	fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}

impl fmt::Display for JsEvaluationError {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

		write!(f, "{}", self.message.as_str())
	}
}



/****************************************************************************************************************
 * The C handler functions that are invoked by external C code, and that in turn invoke relevant Rust handlers. *
 ****************************************************************************************************************/

unsafe extern "C" fn ffi_creation_callback_handler( bw: *mut cbw_BrowserWindow, _data: *mut c_void ) {

	let data_ptr = _data as *mut CreationCallbackData;
	let data = Box::from_raw( data_ptr );

	let handle = BrowserWindowImpl { inner: bw };

	(data.func)( handle, data.data );
}

unsafe extern "C" fn ffi_eval_js_callback_handler( bw: *mut cbw_BrowserWindow, _data: *mut c_void, _result: *const c_char, error: *const cbw_Err ) {

	let data_ptr = _data as *mut EvalJsCallbackData;
	let data = Box::from_raw( data_ptr );

	let (handle, result) = ffi_eval_js_callback_result( bw, _result, error );

	(data.callback)( handle, data.data, result );
}

unsafe extern "C" fn ffi_handler( bw: *mut cbw_BrowserWindow, cmd: cbw_CStrSlice, args: *mut cbw_CStrSlice, arg_count: usize ) {

	let handle = BrowserWindowImpl { inner: bw };

	let data_ptr = (*bw).user_data as *mut UserData;
	let data = Box::from_raw( data_ptr );

	// Convert the command and args to a String and `Vec<&str>`
	let cmd_string: &str = cmd.into();
	let mut args_vec = Vec::with_capacity( arg_count );
	for i in 0..arg_count {
		args_vec.push( (*args.add( i )).into() );
	}

	(*data.func)( handle, cmd_string, args_vec );
}

/// Processes the result received from the C function, and returns it in a Rust Result.
unsafe fn ffi_eval_js_callback_result(
	bw: *mut cbw_BrowserWindow,
	result: *const c_char,
	error: *const cbw_Err
) -> ( BrowserWindowImpl, Result<String, JsEvaluationError> ) {


	// Construct a result value depending on whether the result or error parameters are set
	let result_val: Result<String, JsEvaluationError> = if error.is_null() {
		let result_str = CStr::from_ptr( result ).to_string_lossy().to_owned().to_string();
		Ok( result_str )
	}
	else {
		Err( JsEvaluationError::new( error ) )
	};

	let handle = BrowserWindowImpl { inner: bw };

	// return
	( handle, result_val )
}