use super::application::*;
use super::window::*;

use std::{
	future::Future,
	ptr
};
use unsafe_send_sync::UnsafeSend;



macro_rules! _def_event {
	( $(#[$metas:meta])*, $args_type:ty, $name:ident, $name_async:ident ) => {
		$(#[$metas])*
		pub fn $name<H>( &mut self, handler: H ) -> &mut Self where
			H: FnMut( &$args_type ) + 'static
		{
			self.events.$name.register( handler );
			self
		}

		$(#[$metas])*
		pub fn $name_async<H,F>( &mut self, handler: H ) -> &mut Self where
			H: FnMut( &$args_type ) -> F + 'static,
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
	pub(in crate) borders: bool,
	pub(in crate) events: Box<WindowEvents>,
	pub(in crate) height: i32,
	pub(in crate) minimizable: bool,
	pub(in crate) parent: Option<UnsafeSend<WindowHandle>>,
	pub(in crate) resizable: bool,
	pub(in crate) title: Option<String>,
	pub(in crate) width: i32
}

struct WindowUserData {
	events: Box<WindowEvents>
}



impl WindowBuilder {

	def_event!{ /// Invoked whenever the window closes, whether it was closed by the user or programmatically.
		on_close, on_close_async
	}

	def_event!{ /// Invoked whenever the window resizes
		WindowResizeEventArgs, on_resize, on_resize_async
	}

	/// Sets whether or not the window has borders.
	/// Default is true.
	pub fn borders( &mut self, value: bool ) -> &mut Self {
		self.borders = value;	self
	}

	// TODO: Create a Window struct that can be created with this method.
	fn build( self, app: ApplicationHandle ) {

		// Title
		let title: &str = match self.title.as_ref() {
			None => "",
			Some( t ) => t
		};

		// Convert options to the FFI struct
		let window_options = bw_WindowOptions {
			borders: self.borders,
			minimizable: self.minimizable,
			resizable: self.resizable
		};

		// Put event data into a user data pointer
		let user_data = Box::new( WindowUserData {
			events: unsafe { Box::from_raw( Box::into_raw( self.events ) ) }	// Move ownership of events into WindowUserData
		} );

		// Unwrap the parent ffi handle
		let parent_ffi_handle = match self.parent {
			None => ptr::null(),
			Some( parent ) => parent.ffi_handle
		};

		let _ffi_handle = unsafe { bw_Window_new(
			app.ffi_handle,
			parent_ffi_handle,
			title.into(),
			self.width as _,
			self.height as _,
			&window_options,
			Box::into_raw( user_data ) as _
		) };
	}

	/// Sets the height that the browser window will be created with initially
	pub fn height( &mut self, height: u32 ) -> &mut Self {
		self.height = height as i32;
		self
	}

	/// Sets whether or not the window has a minimize button on the title bar
	/// Default is true
	pub fn minimizable( &mut self, value: bool ) -> &mut Self {
		self.minimizable = value;	self
	}

	/// Sets the opacity of the window.
	/// An opacity of 255 is the default and means the window is fully visible.
	/// An lower opacity means the window will be transparent.
	//pub fn opacity( &mut self, value: u8 ) -> &mut Self {
	//   self.opacity = value;   self
	//}

	/// Configure a parent window.
	/// When a parent window closes, this browser window will close as well.
	/// This could be a reference to a `Browser` or `BrowserThreaded` handle.
	pub fn parent<W>( &mut self, bw: &W ) -> &mut Self where
		W: OwnedWindow
	{
		self.parent = Some( UnsafeSend::new( bw.window_handle() ) );
		self
	}

	pub fn new() -> Self {
		Self {
			borders: true,
			height: -1,
			minimizable: true,
			parent: None,
			resizable: true,
			title: None,
			width: -1,
			events: Box::new( WindowEvents::default() )
		}
	}

	/// Sets the width and height of the browser window
	pub fn size( &mut self, width: u32, height: u32 ) -> &mut Self {
		self.width = width as i32;
		self.height = height as i32;
		self
	}

	/// Sets the title of the window.
	pub fn title<S: Into<String>>( &mut self, title: S ) -> &mut Self {
		self.title = Some( title.into() );
		self
	}


	/// Sets the width that the browser window will be created with initially.
	pub fn width( &mut self, width: u32 ) -> &mut Self {
		self.width = width as i32;
		self
	}

	/// Sets whether or not the window will be resizable.
	/// Default is true.
	pub fn resizable( &mut self, resizable: bool ) -> &mut Self {
		self.resizable = resizable;	self
	}
}