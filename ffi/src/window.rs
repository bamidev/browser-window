#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::common::*;
use super::string::*;

pub enum bw_Window {}

#[repr(C)]
pub struct bw_WindowOptions {
    pub borders: bool,
    pub minimizable: bool,
    pub resizable: bool
}


extern "C" {
    pub fn bw_Window_hide( window: *mut bw_Window );
    pub fn bw_Window_getContentDimensions( window: *mut bw_Window ) -> bw_Dims2D;
    pub fn bw_Window_getOpacity( window: *mut bw_Window ) -> u8;
    pub fn bw_Window_getPosition( window: *mut bw_Window ) -> bw_Pos2D;
    pub fn bw_Window_getTitle( window: *mut bw_Window, title: bw_StrSlice ) -> usize;
    pub fn bw_Window_getWindowDimensions( window: *mut bw_Window ) -> bw_Dims2D;
    pub fn bw_Window_setTitle( window: *mut bw_Window, title: bw_CStrSlice );
    pub fn bw_Window_setContentDimensions( window: *mut bw_Window, dimensions: bw_Dims2D );
    pub fn bw_Window_setOpacity( window: *mut bw_Window, opacity: u8 );
    pub fn bw_Window_setPosition( window: *mut bw_Window, position: bw_Pos2D );
    pub fn bw_Window_setWindowDimensions( window: *mut bw_Window, dimensions: bw_Dims2D );
    pub fn bw_Window_show( window: *mut bw_Window );
}