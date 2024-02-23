use std::{ffi::CStr, fmt};

use browser_window_c::*;

#[derive(Debug)]
pub struct Error(cbw_Err);

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		unsafe {
			let c_msg = cbw_Err_message(&self.0);

			let result = write!(
				f,
				"[{}] {}",
				self.0.code,
				CStr::from_ptr(c_msg)
					.to_str()
					.expect("invalid utf-8 in bw_Err error message")
			);

			cbw_string_freeCstr(c_msg);

			return result;
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}

impl From<cbw_Err> for Error {
	fn from(e: cbw_Err) -> Self { Self(e) }
}
