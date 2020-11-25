mod bindings;

use std::{
	error::Error,
	ffi::CStr,
	fmt,
	ptr,
	slice,
	str
};



pub use bindings::{

	// Appliation
	bw_Application,
	bw_Application_assertCorrectThread,
	bw_Application_dispatch,
	bw_Application_exit,
	bw_Application_exitAsync,
	bw_Application_finish,
	bw_Application_free,
	bw_Application_initialize,
	bw_Application_isRunning,
	bw_Application_run,

	// BrowserWindow
	bw_BrowserWindow,
	bw_BrowserWindow_destroy,
	bw_BrowserWindow_drop,
	bw_BrowserWindow_evalJs,
	bw_BrowserWindow_evalJsThreaded,
	bw_BrowserWindow_getApp,
	bw_BrowserWindow_getUserData,
	bw_BrowserWindow_getWindow,
	bw_BrowserWindow_navigate,
	bw_BrowserWindow_new,
	bw_BrowserWindowCreationCallbackFn,
	bw_BrowserWindowHandlerFn,
	bw_BrowserWindowJsCallbackFn,
	bw_BrowserWindowOptions,
	bw_BrowserWindowSource,

	bw_CStrSlice,

	bw_Dims2D,

	bw_Err,
	bw_Err_free,

	bw_Pos2D,

	bw_StrSlice,
	bw_string_freeCstr,

	// Window
	bw_Window,
	bw_WindowCallbacks,
	bw_WindowDispatchData,
	bw_WindowDispatchFn,
	bw_WindowOptions,
	bw_Window_destroy,
	bw_Window_drop,
	bw_Window_getContentDimensions,
	bw_Window_getOpacity,
	bw_Window_getPosition,
	bw_Window_getTitle,
	bw_Window_getWindowDimensions,
	bw_Window_hide,
	bw_Window_isVisible,
	bw_Window_new,
	bw_Window_setContentDimensions,
	bw_Window_setOpacity,
	bw_Window_setPosition,
	bw_Window_setTitle,
	bw_Window_setWindowDimensions,
	bw_Window_show
};



/**************************************************************
 * Implementations for C structs that are also useful in Rust *
 **************************************************************/

impl bw_CStrSlice {
	pub fn empty() -> Self {
		Self { len: 0, data: ptr::null() }
	}
}

impl From<&str> for bw_CStrSlice {
	fn from( string: &str ) -> Self {
		Self {
			data: string.as_bytes().as_ptr() as _,
			len: string.len() as _
		}
	}
}

impl<'a> Into<&'a str> for bw_CStrSlice {
	fn into( self ) -> &'a str {
		let raw: &[u8] = unsafe { slice::from_raw_parts(self.data as _, self.len as _ ) };

		#[cfg(debug_assertions)]
			return str::from_utf8( raw ).expect("Invalid UTF-8");
		#[cfg(not(debug_assertions))]
			return unsafe { str::from_utf8_unchecked( raw ) };
	}
}

impl Into<String> for bw_CStrSlice {
	fn into( self ) -> String {
		let str: &str = self.into();

		str.to_owned()
	}
}



impl bw_StrSlice {
	pub fn empty() -> Self {
		Self { len: 0, data: ptr::null_mut() }
	}
}

impl<'a> Into<&'a str> for bw_StrSlice {
	fn into( self ) -> &'a str {
		let raw: &[u8] = unsafe { slice::from_raw_parts(self.data as _, self.len as _ ) };

		#[cfg(debug_assertions)]
			return str::from_utf8( raw ).expect("Invalid UTF-8");
		#[cfg(not(debug_assertions))]
			return unsafe { str::from_utf8_unchecked( raw ) };
	}
}

impl Into<String> for bw_StrSlice {
	fn into( self ) -> String {
		let str: &str = self.into();

		str.to_owned()
	}
}

impl Error for bw_Err {
	fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}

impl fmt::Display for bw_Err {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

		unsafe {
			let msg_ptr = (self.alloc_message.unwrap())( self.code, self.data );
			let cstr = CStr::from_ptr( msg_ptr );
			write!(f, "{}", cstr.to_string_lossy().as_ref())
		}
	}
}