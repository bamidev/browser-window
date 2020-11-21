mod builder;

use super::*;

use browser_window_ffi::*;



pub use builder::WindowBuilder;



pub struct WindowHandle {
    pub(in super) ffi_handle: *mut bw_Window
}

pub trait OwnedWindow {
    fn window_handle( &self ) -> WindowHandle;
}




prop!{
/// Gets or sets the title of the window
pub Title<String, &str>( this: WindowHandle ) {
    get => {
        // First obtain string size
        let buf_len = unsafe { bw_Window_getTitle( this.ffi_handle, bw_StrSlice::empty() ) };

        // Allocate buffer and copy string into it
        let mut buf = vec![0u8; buf_len];
        let slice = bw_StrSlice { len: buf_len, data: buf.as_mut_ptr() as _ };
        unsafe { bw_Window_getTitle( this.ffi_handle, slice ) };

        // Convert to String
        slice.into()
    },
    set(val) => {
        let slice: bw_CStrSlice = val.into();
        unsafe { bw_Window_setTitle( this.ffi_handle, slice ) };
    }
} }

impl WindowHandle {
    impl_prop!{ pub title: Title }

    pub(in super) fn new( ffi_handle: *mut bw_Window ) -> Self {
        Self {
            ffi_handle
        }
    }
}