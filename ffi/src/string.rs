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
		unsafe { str::from_utf8_unchecked(
			slice::from_raw_parts( self.data as *const u8, self.len )
		) }
	}
}
