use unsafe_send_sync::UnsafeSend;

use crate::{application::*, core::prelude::*, window::*, HasHandle};


/// Exposes functionality related to constructing a window.
pub struct WindowBuilder {
	pub(crate) borders: bool,
	pub(crate) height: Option<u32>,
	pub(crate) minimizable: bool,
	pub(crate) parent: Option<UnsafeSend<WindowImpl>>,
	pub(crate) resizable: bool,
	pub(crate) title: Option<String>,
	pub(crate) width: Option<u32>,
}

#[allow(dead_code)]
pub type WindowOptions = cbw_WindowOptions;

impl WindowBuilder {
	/// Sets whether or not the window has borders.
	/// Default is true.
	pub fn borders(&mut self, value: bool) { self.borders = value; }

	// TODO: Create a Window struct that can be created with this method.
	#[allow(dead_code)]
	fn build(self, app: ApplicationHandle) {
		// Title
		let title: &str = match self.title.as_ref() {
			None => "",
			Some(t) => t,
		};

		// Convert options to the FFI struct
		let window_options = WindowOptions {
			borders: self.borders,
			minimizable: self.minimizable,
			resizable: self.resizable,
		};

		// Unwrap the parent ffi handle
		let parent_impl_handle = match self.parent {
			None => WindowImpl::default(),
			Some(parent) => (*parent).clone(),
		};

		let _impl_handle = WindowImpl::new(
			app.inner,
			parent_impl_handle,
			title.into(),
			self.width as _,
			self.height as _,
			&window_options,
		);
	}

	/// Sets the height that the browser window will be created with initially
	pub fn height(&mut self, height: u32) { self.height = Some(height); }

	/// Sets whether or not the window has a minimize button on the title bar
	/// Default is true
	pub fn minimizable(&mut self, value: bool) { self.minimizable = value; }

	/// Configure a parent window.
	/// When a parent window closes, this browser window will close as well.
	/// This could be a reference to a `Browser` or `BrowserThreaded` handle.
	pub fn parent<W>(&mut self, bw: &W)
	where
		W: HasHandle<WindowHandle>,
	{
		self.parent = Some(UnsafeSend::new(bw.handle().0.clone()));
	}

	pub fn new() -> Self {
		Self {
			borders: true,
			height: None,
			minimizable: true,
			parent: None,
			resizable: true,
			title: None,
			width: None,
		}
	}

	/// Sets the width and height of the browser window
	pub fn size(&mut self, width: u32, height: u32) {
		self.width = Some(width);
		self.height = Some(height);
	}

	/// Sets the title of the window.
	pub fn title<S: Into<String>>(&mut self, title: S) { self.title = Some(title.into()); }

	/// Sets the width that the browser window will be created with initially.
	pub fn width(&mut self, width: u32) { self.width = Some(width); }

	/// Sets whether or not the window will be resizable.
	/// Default is true.
	pub fn resizable(&mut self, resizable: bool) { self.resizable = resizable; }
}
