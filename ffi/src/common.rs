#![allow(non_camel_case_types)]

#[repr(C)]
pub struct bw_Dims2D {
    pub width: u16,
    pub height: u16
}

#[repr(C)]
pub struct bw_Pos2D {
    pub x: u16,
    pub y: u16
}