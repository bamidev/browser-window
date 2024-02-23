//! This module contains all window related functionality.

mod builder;

pub use builder::WindowBuilder;

use super::{event::Event, prelude::*};

pub type StandardWindowEvent = Event<'static, WindowHandle>;

/// A handle that exposes all windowing functionality.
#[derive(Clone)]
pub struct WindowHandle {
	pub(super) inner: WindowImpl,
}

#[derive(Default)]
pub(crate) struct WindowEvents {
	pub on_close: StandardWindowEvent,
	pub on_destroy: StandardWindowEvent,
	pub on_resize: Event<'static, WindowResizeEventArgs>,
}

pub struct WindowResizeEventArgs {
	handle: WindowHandle,
	new_size: Dims2D,
}

pub trait OwnedWindow {
	fn window_handle(&self) -> WindowHandle;
}

impl WindowHandle {
	impl_prop! { pub content_dimensions: ContentDimensions }

	impl_prop! { pub opacity: Opacity }

	impl_prop! { pub position: Position }

	impl_prop! { pub title: Title }

	impl_prop! { pub window_dimensions: WindowDimensions }

	/// Destroys the window.
	pub fn close(self) {
		self.hide();
		// The window will be dropped because ownership of `self` is taken.
	}

	/// Make the window invisible to the user.
	pub fn hide(&self) {
		self.inner.hide()
	}

	pub(super) fn new(inner: WindowImpl) -> Self {
		Self { inner }
	}

	/// Make the window visible to the user.
	pub fn show(&self) {
		self.inner.show()
	}
}

prop! { /// Gets or sets the width and height of the content of the window.
	ContentDimensions<Dims2D>( this: WindowHandle ) {
		get => this.inner.get_content_dimensions().into(),
		set(val) => this.inner.set_content_dimensions( val.into() )
	}
}

prop! { /// Gets or sets the opacity of the window.
		/// An opacity of 255 means the window is invisible.
		/// An opacity of 0 means the window is completely visible.
		/// Anything in between makes the window transparent.
		///
		/// This feature only works on Windows.
	Opacity<u8>( this: WindowHandle ) {
		get => this.inner.get_opacity(),
		set(val) => this.inner.set_opacity( val )
	}
}

prop! { /// Gets or sets the current position of the window.
	Position<Pos2D>( this: WindowHandle ) {
		get => this.inner.get_position(),
		set(val) => this.inner.set_position( val )
	}
}

prop! { /// Gets or sets the title of the window.
	pub Title<String, &str>( this: WindowHandle ) {
		get => this.inner.get_title(),
		set(val) => this.inner.set_title( val ).into()
	}
}

prop! { /// Gets or sets the current window size including its border and titlebar.
	WindowDimensions<Dims2D>( this: WindowHandle ) {
		get => this.inner.get_window_dimensions().into(),
		set(val) => this.inner.set_window_dimensions( val.into() )
	}
}
