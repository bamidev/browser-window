#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::string::*;

pub enum bw_Window {}

#[repr(C)]
pub struct bw_WindowOptions {
    pub borders: bool,
    pub closable: bool,
    pub minimizable: bool,
    pub opacity: u8,
    pub resizable: bool
}


extern "C" {
    pub fn bw_Window_getTitle( bw: *mut bw_Window, title: bw_StrSlice ) -> usize;
    pub fn bw_Window_setTitle( bw: *mut bw_Window, title: bw_CStrSlice );
}