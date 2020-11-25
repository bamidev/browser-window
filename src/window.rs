//! This module contains all window related functionality.

mod builder;

use super::*;
use super::common::{Dims2D, Pos2D};

use browser_window_ffi::*;



pub use builder::WindowBuilder;



#[derive(Clone, Copy)]
pub struct WindowHandle {
	pub(in super) ffi_handle: *mut bw_Window
}

pub trait OwnedWindow {
	fn window_handle( &self ) -> WindowHandle;
}



impl WindowHandle {
	impl_prop!{ pub content_dimensions: ContentDimensions }
	impl_prop!{ pub opacity: Opacity }
	impl_prop!{ pub position: Position }
	impl_prop!{ pub title: Title }
	impl_prop!{ pub window_dimensions: WindowDimensions }

	/// Destroys the window.
	pub fn close( self ) {
		self.hide();
		// The window will be dropped because ownership of `self` is taken.
	}

	/// Make the window invisible to the user.
	pub fn hide( &self ) {
		unsafe { bw_Window_hide( self.ffi_handle ) };
	}

	pub(in super) fn new( ffi_handle: *mut bw_Window ) -> Self {
		Self {
			ffi_handle
		}
	}

	/// Make the window visible to the user.
	pub fn show( &self ) { eprintln!("TEST SHOW");
		unsafe { bw_Window_show( self.ffi_handle ) }; eprintln!("TEST SHOW");
	}
}



prop! { /// Gets or sets the width and height of the content of the window.
	ContentDimensions<Dims2D>( this: WindowHandle ) {
		get => unsafe{ bw_Window_getContentDimensions( this.ffi_handle ) }.into(),
		set(val) => unsafe { bw_Window_setContentDimensions( this.ffi_handle, val.into() ) }
	}
}

prop! { /// Gets or sets the opacity of the window.
        /// An opacity of 255 means the window is invisible.
        /// An opacity of 0 means the window is completely visible.
        /// Anything in between makes the window transparent.
	Opacity<u8>( this: WindowHandle ) {
		get => unsafe { bw_Window_getOpacity( this.ffi_handle ) }.into(),
		set(val) => unsafe { bw_Window_setOpacity( this.ffi_handle, val.into() ) }
	}
}

prop! { /// Gets or sets the current position of the window.
	Position<Pos2D>( this: WindowHandle ) {
		get => unsafe { bw_Window_getPosition( this.ffi_handle ) }.into(),
		set(val) => unsafe { bw_Window_setPosition( this.ffi_handle, val.into() ) }
	}
}

prop!{ /// Gets or sets the title of the window.
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
	}
}

prop! { /// Gets or sets the current window size including its border and titlebar.
	WindowDimensions<Dims2D>( this: WindowHandle ) {
		get => unsafe{ bw_Window_getWindowDimensions( this.ffi_handle ) }.into(),
		set(val) => unsafe { bw_Window_setWindowDimensions( this.ffi_handle, val.into() ) }
	}
}
