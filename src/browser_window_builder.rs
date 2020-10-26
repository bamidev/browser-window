use browser_window_ffi::*;

use std::marker::PhantomData;

use crate::application::{Application, ApplicationAsync, ApplicationHandle};
use crate::browser_window::{BrowserWindow, BrowserWindowHandle, BrowserWindowInner, BrowserWindowAsync};

use std::{
	mem,
	ops::Deref,
	ptr,
	sync::Arc
};



/// The type of content to display in a browser window
pub enum Source {
	Url( String ),
	Html( String )
}

/// Allows building a BrowserWindow instance
pub struct BrowserWindowBuilder {

	parent: Option<BrowserWindowHandle>,
	source: Source,
	title: Option<String>,
	width: Option<u32>,
	height: Option<u32>,
	handler: Option<Box<dyn FnMut(BrowserWindowHandle, &str, &[&str]) + Send>>
}



impl BrowserWindowBuilder {

	/// Configure a closure that can be invoked from within JavaScript
	/// The closure's first parameter specifies a command name.
	/// The closure's second parameter specifies an array of string arguments.
	pub fn handler<H>( mut self, handler: H ) -> Self where
		H: FnMut(BrowserWindowHandle, &str, &[&str]) + Send + 'static
	{
		self.handler = Some( Box::new( handler ) );
		self
	}

	/// Configure a parent window.
	/// When a parent window closes, this browser window will close as well.
	pub fn parent<B>( mut self, bw: &B ) -> Self where
		B: Deref<Target=BrowserWindowHandle>
	{
		self.parent = Some( (**bw).clone() );
		self
	}

	/// Creates an instance of a browser window builder.
	///
	/// # Arguments
	/// * `source` - The content that will be displayed in the browser window.
	pub fn new( source: Source ) -> Self {
		Self {
			parent: None,
			source: source,
			handler: None,
			title: None,
			width: None,
			height: None
		}
	}

	/// Sets the title of the window
	///
	/// # Arguments
	/// * `title` - The text that will be displayed in the title bar
	pub fn title( mut self, title: String ) -> Self {
		self.title = Some( title );
		self
	}

	/// Sets the width that the browser window will be created with initially
	///
	/// # Arguments
	/// * `width` - Width in pixels
	pub fn width( mut self, width: u32 ) -> Self {
		self.width = Some( width );
		self
	}

	/// Sets the height that the browser window will be created with initially
	///
	/// # Arguments
	/// * `height` - Height in pixels
	pub fn height( mut self, height: u32 ) -> Self {
		self.height = Some( height );
		self
	}

	/// Creates the browser window, and immediately displays it to the user.
	/// A browser window handle is returned.
	///
	/// # Arguments
	/// * `app` - A reference to an application handle that this browser window can spawn into
	pub fn spawn( self, app: &Application ) -> BrowserWindow {
		BrowserWindow {
			inner: self._spawn( (*app.inner).clone() ),
			_not_send: PhantomData
		}
	}

	fn _spawn( self, app: ApplicationHandle ) -> Arc<BrowserWindowInner> {

		match self {
			BrowserWindowBuilder { parent, source, title, width, height, handler } => {

				// Parent
				let parent_handle = match parent {
					None => ptr::null(),
					Some( p ) => p._ffi_handle
				};

				// Source
				let csource = match &source {	// Use a reference, we want source to live until the end of the function because bw_BrowserWindowSource holds a reference to its internal string.
					Source::Url( url ) => { bw_BrowserWindowSource {
						data: url.as_str().into(),
						is_html: false
					} },
					Source::Html( html ) => { bw_BrowserWindowSource {
						data: html.as_str().into(),
						is_html: true
					} }
				};

				let title_ptr = match title.as_ref() {
					None => "Browser Window".into(),
					Some( t ) => t.as_str().into()
				};

				// Width and height use -1 as the 'use default' option within the c code
				let w: i32 = match width {
					Some(w) => w as i32,
					None => -1
				};
				let h: i32 = match height {
					Some(h) => h as i32,
					None => -1
				};

				let user_data = Box::into_raw( Box::new(
					BrowserWindowHandlerData {
						func: match handler {
							None => Box::new(|_,_,_|{}),
							Some(f) => Box::new(f)
						}
					}
				) );

				// TODO: Expose these options in the BrowserWindowBuilder
				let window_options = bw_WindowOptions {
					maximizable: true,
					minimizable: true,
					resizable: true,
					closable: true,
					borders: true,
					is_popup: true
				};
				let other_options = bw_BrowserWindowOptions {
					dev_tools: true
				};

				let ffi_handle = unsafe { bw_BrowserWindow_new(
					app._ffi_handle.clone(),
					parent_handle,
					csource,
					title_ptr,
					w,
					h,
					&window_options,
					&other_options,
					ffi_window_invoke_handler,
					user_data as _
				) };

				Arc::new( BrowserWindowInner {
					app: app,
					handle: BrowserWindowHandle { _ffi_handle: ffi_handle }
				} )
			}
		}
	}

	/// Same as spawn, but asynchronous.
	/// Returns an asynchronous handle that is available immediately.
	/// However, this function may return before the window is actually shown.
	/// Nevertheless, the handle is valid.
	///
	/// # Arguments
	/// * `app` - An async application handle.
	pub async fn spawn_async( self, app: &ApplicationAsync ) -> BrowserWindowAsync {

		let app_inner: ApplicationHandle = (*app.inner).clone();

		let bw_inner = *app.dispatch(move |_| {
			self._spawn( app_inner )
		}).await;

		BrowserWindowAsync {
			inner: bw_inner
		}
	}
}

/// The data that is passed to the C FFI handler function
struct BrowserWindowHandlerData {
	func: Box<dyn FnMut( BrowserWindowHandle, &str, &[&str])>
}



/// Takes the arguments received from the C FFI handler callback, and converts it to a vector of strings
fn args_to_vec<'a>( args: *const bw_CStrSlice, args_count: usize ) -> Vec<&'a str> {

	let mut vec = Vec::<&str>::with_capacity( args_count );

	for i in 0..args_count {
		let str_ref: &str = unsafe { (*args.offset(i as _)).into() };

		vec.push( str_ref );
	}

	vec
}

/// This external C funtion will be invoked by the underlying implementation browser-window when it is invoked in JavaScript
extern "C" fn ffi_window_invoke_handler( inner_handle: *mut bw_BrowserWindow, _command: bw_CStrSlice, _args: *const bw_CStrSlice, args_count: usize ) {

	unsafe {
		let data_ptr: *mut BrowserWindowHandlerData = mem::transmute( bw_BrowserWindow_get_user_data( inner_handle ) );
		let data: &mut BrowserWindowHandlerData = &mut *data_ptr;

		match data {
			BrowserWindowHandlerData{ func } => {
				let outer_handle = BrowserWindowHandle::from_ptr( inner_handle );

				let args = args_to_vec( _args, args_count );

				func( outer_handle, _command.into(), &*args );
			}
		}
	}
}
