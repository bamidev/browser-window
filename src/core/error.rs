use crate::prelude::*;

use std::{
	ffi::CStr,
	fmt
};



#[derive(Debug)]
pub struct CbwError (cbw_Err);


pub type CbwResult<T> = Result<T, CbwError>;



impl fmt::Display for CbwError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		unsafe {
			let c_msg = cbw_Err_message( &self.0 );

			let result = write!(f, "[{}] {}", self.0.code, CStr::from_ptr(c_msg).to_str().expect("invalid utf-8 in bw_Err error message"));

			cbw_string_freeCstr(c_msg);

			return result;
		}
	}
}

impl std::error::Error for CbwError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}

impl From<cbw_Err> for CbwError {
	fn from(e: cbw_Err) -> Self {
		Self (e)
	}
}