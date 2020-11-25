#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::error::Error;
use std::ffi::CStr;
use std::fmt;
use std::os::raw::*;



pub type bw_ErrCode = c_uint;
pub type bw_ErrMsgFn = extern "C" fn( code: c_uint, data: *const c_void ) -> *mut c_char;




#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct bw_Err {
	pub code: c_uint,
	pub data: *const c_void,
	pub alloc_message: bw_ErrMsgFn
}
// We assume bw_Err is Send, because data is supposed to be set once and not messed with until freed
unsafe impl Send for bw_Err {}



impl Error for bw_Err {
	fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}

impl fmt::Display for bw_Err {

	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

		unsafe {
			let msg_ptr = (self.alloc_message)( self.code, self.data );
			let cstr = CStr::from_ptr( msg_ptr );
			write!(f, "{}", cstr.to_string_lossy().as_ref())
		}
	}
}



extern "C" {
	pub fn bw_Err_free( err: *mut bw_Err );
}
