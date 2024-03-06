//! This module contains all window related functionality.

mod builder;

use super::prelude::*;

pub use builder::WindowBuilder;
pub use super::core::window::WindowExt;



/// A handle that exposes all windowing functionality.
pub struct WindowHandle(
	pub(super) WindowImpl,
);

pub trait HasWindowHandle {
	fn window_handle(&self) -> &WindowHandle;
}

impl WindowHandle {
	pub(crate) unsafe fn clone(&self) -> Self {
		Self (self.0.clone())
	}

	pub(super) fn new(inner: WindowImpl) -> Self { Self(inner) }

	pub fn content_dimensions(&self) -> Dims2D { self.0.content_dimensions() }
	pub fn opacity(&self) -> u8 { self.0.opacity() }
	pub fn position(&self) -> Pos2D { self.0.position() }
	pub fn title(&self) -> String { self.0.title() }
	pub fn window_dimensions(&self) -> Dims2D { self.0.window_dimensions() }

	pub fn hide(&self) { self.0.hide(); }

	pub fn set_content_dimensions(&self, dimensions: Dims2D) { self.0.set_content_dimensions(dimensions); }
	pub fn set_opacity(&self, opacity: u8) { self.0.set_opacity(opacity); }
	pub fn set_position(&self, position: Pos2D) { self.0.set_position(position); }
	pub fn set_title(&self, title: &str) { self.0.set_title(title); }
	pub fn set_window_dimensions(&self, dimensions: Dims2D) { self.0.set_window_dimensions(dimensions); }

	pub fn show(&self) { self.0.show(); }
}
