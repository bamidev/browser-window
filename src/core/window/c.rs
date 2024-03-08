//! This module implements the `Window` trait with the corresponding function
//! definitions found in the C code base of `browser-window-c`. All functions
//! are basically wrapping the FFI provided by crate `browser-window-c`.

use std::{os::raw::c_char, ptr};

use super::{WindowExt, WindowOptions};
use crate::{core::application::ApplicationImpl, prelude::*};

#[derive(Clone)]
pub struct WindowImpl {
	pub(crate) inner: *mut cbw_Window,
}

impl WindowImpl {
	pub fn new(
		app: ApplicationImpl, parent: Self, title: &str, width: Option<u32>, height: Option<u32>,
		options: &WindowOptions,
	) -> Self {
		let str_slice: cbw_CStrSlice = title.into();

		let w = match width {
			None => -1i32,
			Some(x) => x as i32,
		};
		let h = match height {
			None => -1i32,
			Some(x) => x as i32,
		};

		let handle = unsafe { cbw_Window_new(app.inner, parent.inner, str_slice, w, h, options) };

		// Return
		Self { inner: handle }
	}
}

impl Default for WindowImpl {
	fn default() -> Self {
		Self {
			inner: ptr::null_mut(),
		}
	}
}

impl WindowExt for WindowImpl {
	fn app(&self) -> ApplicationImpl {
		ApplicationImpl {
			inner: unsafe { (*self.inner).app },
		}
	}

	fn close(&self) { unsafe { cbw_Window_close(self.inner) } }

	fn free(&self) { unsafe { cbw_Window_free(self.inner) } }

	fn content_dimensions(&self) -> Dims2D {
		unsafe { cbw_Window_getContentDimensions(self.inner) }
	}

	fn opacity(&self) -> u8 { unsafe { cbw_Window_getOpacity(self.inner) } }

	fn position(&self) -> Pos2D { unsafe { cbw_Window_getPosition(self.inner) } }

	fn title(&self) -> String {
		// First obtain string size
		let mut buf: *mut c_char = ptr::null_mut();
		let buf_len = unsafe { cbw_Window_getTitle(self.inner, &mut buf) };

		let slice = cbw_StrSlice {
			data: buf,
			len: buf_len,
		};

		unsafe { cbw_string_freeCstr(buf) };

		// Convert to String
		slice.into()
	}

	fn window_dimensions(&self) -> Dims2D { unsafe { cbw_Window_getWindowDimensions(self.inner) } }

	fn hide(&self) { unsafe { cbw_Window_hide(self.inner) } }

	fn set_content_dimensions(&self, dimensions: Dims2D) {
		unsafe { cbw_Window_setContentDimensions(self.inner, dimensions) }
	}

	fn set_opacity(&self, opacity: u8) { unsafe { cbw_Window_setOpacity(self.inner, opacity) } }

	fn set_position(&self, position: Pos2D) {
		unsafe { cbw_Window_setPosition(self.inner, position) }
	}

	fn set_title(&self, title: &str) {
		let slice: cbw_CStrSlice = title.into();
		unsafe { cbw_Window_setTitle(self.inner, slice) };
	}

	fn set_user_data(&self, user_data: *mut ()) {
		unsafe {
			(*self.inner).user_data = user_data as _;
		}
	}

	fn set_window_dimensions(&self, dimensions: Dims2D) {
		unsafe { cbw_Window_setWindowDimensions(self.inner, dimensions) }
	}

	fn show(&self) { unsafe { cbw_Window_show(self.inner) } }
}
