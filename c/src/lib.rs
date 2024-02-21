mod bindings;

use std::{error::Error, ffi::CStr, fmt, ptr, slice, str};

pub use crate::bindings::*;

/**************************************************************
 * Implementations for C structs that are also useful in Rust *
 *********************************************************** */

impl cbw_CStrSlice {
	pub fn empty() -> Self {
		Self {
			len: 0,
			data: ptr::null(),
		}
	}
}

impl From<&str> for cbw_CStrSlice {
	fn from(string: &str) -> Self {
		Self {
			data: string.as_bytes().as_ptr() as _,
			len: string.len() as _,
		}
	}
}

impl<'a> Into<&'a str> for cbw_CStrSlice {
	fn into(self) -> &'a str {
		let raw: &[u8] = unsafe { slice::from_raw_parts(self.data as _, self.len as _) };

		#[cfg(debug_assertions)]
		return str::from_utf8(raw).expect("Invalid UTF-8");
		#[cfg(not(debug_assertions))]
		return unsafe { str::from_utf8_unchecked(raw) };
	}
}

impl Into<String> for cbw_CStrSlice {
	fn into(self) -> String {
		let str: &str = self.into();

		str.to_owned()
	}
}

impl cbw_StrSlice {
	pub fn empty() -> Self {
		Self {
			len: 0,
			data: ptr::null_mut(),
		}
	}
}

impl<'a> Into<&'a str> for cbw_StrSlice {
	fn into(self) -> &'a str {
		let raw: &[u8] = unsafe { slice::from_raw_parts(self.data as _, self.len as _) };

		#[cfg(debug_assertions)]
		return str::from_utf8(raw).expect("Invalid UTF-8");
		#[cfg(not(debug_assertions))]
		return unsafe { str::from_utf8_unchecked(raw) };
	}
}

impl Into<String> for cbw_StrSlice {
	fn into(self) -> String {
		let str: &str = self.into();

		str.to_owned()
	}
}

impl Error for cbw_Err {
	fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}

impl fmt::Display for cbw_Err {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		unsafe {
			let msg_ptr = (self.alloc_message.unwrap())(self.code, self.data);
			let cstr = CStr::from_ptr(msg_ptr);
			write!(f, "{}", cstr.to_string_lossy().as_ref())
		}
	}
}
