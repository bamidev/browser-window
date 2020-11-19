mod builder;

use browser_window_ffi::*;



pub use builder::WindowBuilder;



pub struct WindowHandle {
    pub(in super) ffi_handle: *mut bw_Window
}

pub trait OwnedWindow {
    fn window_handle( &self ) -> WindowHandle;
}



impl WindowHandle {
    pub(in super) fn new( ffi_handle: *mut bw_Window ) -> Self {
        Self {
            ffi_handle
        }
    }
}