#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::os::raw::*;



pub enum bw_Application {}
type bw_ApplicationDispatchFn = extern "C" fn( app: *mut bw_Application, data: *mut c_void );
type bw_ApplicationReadyFn = bw_ApplicationDispatchFn;



extern "C" {
	pub fn bw_Application_dispatch( app: *mut bw_Application, func: bw_ApplicationDispatchFn, data: *mut c_void );
	pub fn bw_Application_exit( app: *mut bw_Application, result: c_int );
	pub fn bw_Application_exitAsync( app: *mut bw_Application, result: c_int );
	pub fn bw_Application_finish( app: *mut bw_Application );
	pub fn bw_Application_start( argc: c_int, argv: *mut *mut c_char ) -> *mut bw_Application;
	pub fn bw_Application_run( app: *mut bw_Application, on_ready: bw_ApplicationReadyFn, user_data: *mut c_void ) -> c_int;
}
