#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub enum bw_Window {}

#[repr(C)]
pub struct bw_WindowOptions {
    pub borders: bool,
    pub closable: bool,
    pub minimizable: bool,
    pub opacity: u8,
    pub resizable: bool
}