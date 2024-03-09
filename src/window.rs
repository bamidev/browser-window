//! This module contains all window related functionality.

mod builder;

pub use builder::WindowBuilder;

pub use super::core::window::WindowExt;
use super::prelude::*;


/// A handle that exposes all windowing functionality.
pub struct WindowHandle(pub(super) WindowImpl);


impl WindowHandle {
	#[cfg(feature = "threadsafe")]
	pub(crate) unsafe fn clone(&self) -> Self { Self(self.0.clone()) }

	pub(super) fn new(inner: WindowImpl) -> Self { Self(inner) }

	pub fn content_dimensions(&self) -> Dims2D { self.0.content_dimensions() }

	pub fn opacity(&self) -> u8 { self.0.opacity() }

	pub fn position(&self) -> Pos2D { self.0.position() }

	pub fn title(&self) -> String { self.0.title() }

	pub fn window_dimensions(&self) -> Dims2D { self.0.window_dimensions() }

	/// Hides the window.
	/// Keep in mind that hiding the window is not the same as closing it.
	/// Hiding the window will keep it's resources alive.
	/// If the window is hidden, and all window handles are gone, the memory is
	/// effectively leaked.
	pub fn hide(&self) { self.0.hide(); }

	pub fn set_content_dimensions(&self, dimensions: Dims2D) {
		self.0.set_content_dimensions(dimensions);
	}

	pub fn set_opacity(&self, opacity: u8) { self.0.set_opacity(opacity); }

	pub fn set_position(&self, position: Pos2D) { self.0.set_position(position); }

	pub fn set_title(&self, title: &str) { self.0.set_title(title); }

	pub fn set_window_dimensions(&self, dimensions: Dims2D) {
		self.0.set_window_dimensions(dimensions);
	}

	/// Shows a window if it was hidden.
	/// Windows that were just created are hidden to start.
	/// This method is necessary to show it to the user.
	pub fn show(&self) { self.0.show(); }
}
