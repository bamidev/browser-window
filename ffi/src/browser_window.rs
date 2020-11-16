#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::os::raw::*;

use super::application::bw_Application;
use super::err::bw_Err;
use super::string::bw_CStrSlice;



pub enum bw_BrowserWindow {}
pub type bw_BrowserWindowCreationCallbackFn = unsafe extern "C" fn ( bw: *mut bw_BrowserWindow, data: *mut c_void );
pub type bw_BrowserWindowDispatchFn = unsafe extern "C" fn( bw: *mut bw_BrowserWindow, data: *mut c_void );
pub type bw_BrowserWindowHandlerFn = unsafe extern "C" fn( bw: *mut bw_BrowserWindow, cmd: bw_CStrSlice, args: *const bw_CStrSlice, args_count: usize );
pub type bw_BrowserWindowEvalJsCallbackFn = unsafe extern "C" fn( bw: *mut bw_BrowserWindow, data: *mut c_void, js: *const c_char, error: *const bw_Err );



#[repr(C)]
pub struct bw_BrowserWindowOptions {
	pub dev_tools: bool,
	pub resource_path: bw_CStrSlice
}

#[repr(C)]
pub struct bw_BrowserWindowSource {
	pub data: bw_CStrSlice,
	pub is_html: bool
}

#[repr(C)]
pub struct bw_WindowOptions {
	pub minimizable: bool,
	pub resizable: bool,
	pub closable: bool,
	pub borders: bool
}



extern "C" {
	pub fn bw_BrowserWindow_close( bw: *mut bw_BrowserWindow );
	pub fn bw_BrowserWindow_dispatch( bw: *mut bw_BrowserWindow, func: bw_BrowserWindowDispatchFn, data: *mut c_void );
	pub fn bw_BrowserWindow_drop( bw: *mut bw_BrowserWindow );
	pub fn bw_BrowserWindow_evalJs( bw: *mut bw_BrowserWindow, js: bw_CStrSlice, callback: bw_BrowserWindowEvalJsCallbackFn, cb_data: *mut c_void );
	pub fn bw_BrowserWindow_evalJsThreaded( bw: *mut bw_BrowserWindow, js: bw_CStrSlice, callback: bw_BrowserWindowEvalJsCallbackFn, cb_data: *mut c_void );
	pub fn bw_BrowserWindow_getApp( bw: *mut bw_BrowserWindow ) -> *mut bw_Application;
	pub fn bw_BrowserWindow_getUserData( bw: *mut bw_BrowserWindow ) -> *mut c_void;
	pub fn bw_BrowserWindow_navigate( bw: *mut bw_BrowserWindow, url: bw_CStrSlice ) -> bw_Err;
	pub fn bw_BrowserWindow_new(
		app: *mut bw_Application,
		parent: *const bw_BrowserWindow,
		source: bw_BrowserWindowSource,
		title: bw_CStrSlice,
		width: c_int,
		height: c_int,
		window_options: *const bw_WindowOptions,
		options: *const bw_BrowserWindowOptions,
		handler: bw_BrowserWindowHandlerFn,
		user_data: *mut c_void,
		callback: bw_BrowserWindowCreationCallbackFn,
		callback_data: *mut c_void
	);
}
