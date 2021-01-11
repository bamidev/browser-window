#![allow(non_snake_case)]

use std::os::raw::*;



#[no_mangle]
pub extern "C" fn bw_BrowserWindowImpl_new(
	bw: *mut bw_BrowserWindow,
	source: bw_BrowserWindowSource,
	width: c_int,
	height: c_int,
	options: *const bw_BrowserWindowOptions,
	callback: bw_BrowserWindowCreationCallbackFn,
	callback_data: *mut c_void
) {
	//Sprintln!("HOI")
}