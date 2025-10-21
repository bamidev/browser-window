#[cfg(not(feature = "gtk"))]
mod c;
#[cfg(feature = "gtk")]
mod gtk;

use std::{ffi::c_void, ptr};

#[cfg(not(feature = "gtk"))]
pub use c::WindowImpl;
#[cfg(feature = "gtk")]
pub use gtk::WindowImpl;

use crate::prelude::*;

pub trait WindowExt: Clone {
	fn app(&self) -> ApplicationImpl;

	fn close(&self);
	fn free(&self);

	fn content_dimensions(&self) -> Dims2D;
	fn opacity(&self) -> u8;
	fn position(&self) -> Pos2D;
	fn title(&self) -> String;
	fn window_dimensions(&self) -> Dims2D;

	fn hide(&self);

	fn set_content_dimensions(&self, dimensions: Dims2D);
	fn set_opacity(&self, opacity: u8);
	fn set_position(&self, position: Pos2D);
	fn set_title(&self, title: &str);
	fn set_user_data(&self, user_data: *mut ());
	fn set_window_dimensions(&self, dimensions: Dims2D);

	fn show(&self);
}

pub type WindowOptions = cbw_WindowOptions;
