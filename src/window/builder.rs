use std::future::Future;

use unsafe_send_sync::UnsafeSend;

use crate::{application::*, core::prelude::*, window::*};

macro_rules! _def_event {
	( $(#[$metas:meta])*, $args_type:ty, $name:ident, $name_async:ident ) => {
		$(#[$metas])*
		#[cfg(not(feature = "threadsafe"))]
		pub fn $name<H>( &mut self, handler: H ) -> &mut Self where
			H: FnMut( &$args_type ) + 'static
		{
			self.events.$name.register( handler );
			self
		}

		$(#[$metas])*
		#[cfg(feature = "threadsafe")]
		pub fn $name<H>( &mut self, handler: H ) -> &mut Self where
			H: FnMut( &$args_type ) + Send + 'static
		{
			self.events.$name.register( handler );
			self
		}

		$(#[$metas])*
		#[cfg(not(feature = "threadsafe"))]
		pub fn $name_async<H,F>( &mut self, handler: H ) -> &mut Self where
			H: FnMut( &$args_type ) -> F + 'static,
			F: Future<Output=()> + 'static
		{
			self.events.$name.register_async( handler );
			self
		}

		$(#[$metas])*
		#[cfg(feature = "threadsafe")]
		pub fn $name_async<H,F>( &mut self, handler: H ) -> &mut Self where
			H: FnMut( &$args_type ) -> F + Send + 'static,
			F: Future<Output=()> + 'static
		{
			self.events.$name.register_async( handler );
			self
		}
	};
}

macro_rules! def_event {
	( $(#[$metas:meta])* $args_type:ty, $name:ident, $name_async:ident ) => {
		_def_event!( $(#[$metas])*, $args_type, $name, $name_async );
	};

	( $(#[$metas:meta])* $name:ident, $name_async:ident ) => {
		_def_event!( $(#[$metas])*, WindowHandle, $name, $name_async );
	};
}

/// Exposes functionality related to constructing a window.
pub struct WindowBuilder {
	pub(crate) borders: bool,
	pub(crate) events: Box<WindowEvents>,
	pub(crate) height: Option<u32>,
	pub(crate) minimizable: bool,
	pub(crate) parent: Option<UnsafeSend<WindowHandle>>,
	pub(crate) resizable: bool,
	pub(crate) title: Option<String>,
	pub(crate) width: Option<u32>,
}

struct WindowUserData {
	events: Box<WindowEvents>,
}

#[allow(dead_code)]
pub type WindowOptions = cbw_WindowOptions;

impl WindowBuilder {
	def_event! { /// Invoked whenever the window closes, whether it was closed by the user or programmatically.
		#[doc(hidden)]
		on_close, on_close_async
	}

	def_event! { /// Invoked whenever the window resizes
		#[doc(hidden)]
		WindowResizeEventArgs, on_resize, on_resize_async
	}

	/// Sets whether or not the window has borders.
	/// Default is true.
	pub fn borders(&mut self, value: bool) -> &mut Self {
		self.borders = value;
		self
	}

	// TODO: Create a Window struct that can be created with this method.
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

		// Put event data into a user data pointer
		let user_data = Box::new(WindowUserData {
			events: unsafe { Box::from_raw(Box::into_raw(self.events)) }, /* Move ownership of
			                                                               * events into
			                                                               * WindowUserData */
		});

		// Unwrap the parent ffi handle
		let parent_impl_handle = match self.parent {
			None => WindowImpl::default(),
			Some(parent) => parent.inner.clone(),
		};

		let _impl_handle = WindowImpl::new(
			app.inner,
			parent_impl_handle,
			title.into(),
			self.width as _,
			self.height as _,
			&window_options,
			Box::into_raw(user_data) as _,
		);
	}

	/// Sets the height that the browser window will be created with initially
	pub fn height(&mut self, height: u32) -> &mut Self {
		self.height = Some(height);
		self
	}

	/// Sets whether or not the window has a minimize button on the title bar
	/// Default is true
	pub fn minimizable(&mut self, value: bool) -> &mut Self {
		self.minimizable = value;
		self
	}

	/// Configure a parent window.
	/// When a parent window closes, this browser window will close as well.
	/// This could be a reference to a `Browser` or `BrowserThreaded` handle.
	pub fn parent<W>(&mut self, bw: &W) -> &mut Self
	where
		W: OwnedWindow,
	{
		self.parent = Some(UnsafeSend::new(bw.window_handle()));
		self
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
			events: Box::new(WindowEvents::default()),
		}
	}

	/// Sets the width and height of the browser window
	pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
		self.width = Some(width);
		self.height = Some(height);
		self
	}

	/// Sets the title of the window.
	pub fn title<S: Into<String>>(&mut self, title: S) -> &mut Self {
		self.title = Some(title.into());
		self
	}

	/// Sets the width that the browser window will be created with initially.
	pub fn width(&mut self, width: u32) -> &mut Self {
		self.width = Some(width);
		self
	}

	/// Sets whether or not the window will be resizable.
	/// Default is true.
	pub fn resizable(&mut self, resizable: bool) -> &mut Self {
		self.resizable = resizable;
		self
	}
}
