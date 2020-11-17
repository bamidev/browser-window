#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{
	os::raw::*,
	slice,
	str
};



#[repr(C)]
#[derive(Copy)]
pub struct bw_CStrSlice {
	pub len: usize,
	pub data: *const c_char
}



extern "C" { pub fn bw_string_freeCstr( str: *const c_char ); }



impl Clone for bw_CStrSlice {
	fn clone( &self ) -> Self {
		panic!("bw_CStrSlice is not actually supposed to be Clone!");
	}
}

impl From<&str> for bw_CStrSlice {
	fn from( string: &str ) -> Self {
		Self {
			data: string.as_bytes().as_ptr() as *const c_char,
			len: string.len()
		}
	}
}

impl<'a> Into<&'a str> for bw_CStrSlice {
	fn into( self ) -> &'a str {
		let raw: &[u8] = unsafe { slice::from_raw_parts(self.data as _, self.len ) };

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
