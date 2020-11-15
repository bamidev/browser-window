use browser_window_ffi::*;

use std::ffi::*;

use crate::application::{Application, ApplicationThreaded};
use crate::browser::*;

use std::{
	mem,
	ops::Deref,
	ptr
};



/// The type of content to display in a browser window
pub enum Source {
	Url( String ),
	Html( String )
}

/// Used to create a `Browser` or `BrowserThreaded` instance.
pub struct BrowserBuilder {

	parent: Option<BrowserHandle>,
	source: Source,
	title: Option<String>,
	width: Option<u32>,
	height: Option<u32>,
	handler: Option<Box<dyn FnMut(BrowserHandle, &str, &[&str]) + Send>>,

	borders: bool,
	dev_tools: bool,
	minimizable: bool,
	resizable: bool
}



impl BrowserBuilder {

	/// Sets whether or not the window has borders.
	/// Default is true.
	pub fn borders( mut self, value: bool ) -> Self {
		self.borders = value;	self
	}

	/// Sets whether or not an extra window with developer tools will be opened together with this browser.
	/// When in debug mode the default is `true`.
	/// When in release mode the default is `false`.
	pub fn dev_tools( mut self, enabled: bool ) -> Self {
		self.dev_tools = enabled;	self
	}

	/// Configure a closure that can be invoked from within JavaScript.
	/// The closure's second parameter specifies a command name.
	/// The closure's third parameter specifies an array of string arguments.
	pub fn handler<H>( mut self, handler: H ) -> Self where
		H: FnMut(BrowserHandle, &str, &[&str]) + Send + 'static
	{
		self.handler = Some( Box::new( handler ) );
		self
	}

	/// Sets whether or not the window has a minimize button on the title bar
	/// Default is true
	pub fn minimizable( mut self, value: bool ) -> Self {
		self.minimizable = value;	self
	}

	/// Configure a parent window.
	/// When a parent window closes, this browser window will close as well.
	/// This could be a reference to a `Browser` or `BrowserThreaded` handle.
	pub fn parent<B>( mut self, bw: &B ) -> Self where
		B: OwnedBrowser
	{
		self.parent = Some( bw.handle() );
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
			height: None,

			borders: true,
			dev_tools: cfg!(debug_assertions),
			minimizable: true,
			resizable: true
		}
	}

	/// Sets the title of the window
	///
	/// # Arguments
	/// * `title` - The text that will be displayed in the title bar
	pub fn title<S: Into<String>>( mut self, title: S ) -> Self {
		self.title = Some( title.into() );
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

	/// Sets whether or not the window will be resizable
	/// Default is true
	pub fn resizable( mut self, resizable: bool ) -> Self {
		self.resizable = resizable;	self
	}

	/// Creates the browser window.
	///
	/// # Arguments
	/// * `app` - An application handle that this browser window can spawn into
	pub async fn build( self, app: Application ) -> Browser
	{
		let (tx, rx) = oneshot::channel::<BrowserHandle>();

		self._build( app, move |handle| {

			if let Err(_) = tx.send( handle ) {
				panic!("Unable to send browser handle back")
			}
		} );

		Browser::new( rx.await.unwrap() )
	}

	/// Same as build, but gives back a browser handle that is thread-safe.
	///
	/// # Arguments
	/// * `app` - An thread-safe application handle.
	pub async fn build_threaded( self, app: ApplicationThreaded ) -> Result<BrowserThreaded, DelegateError> {

		let (tx, rx) = oneshot::channel::<BrowserHandle>();

		// We need to dispatch the spawning of the browser to the GUI thread
		app.delegate(|app_handle| {

			self._build(app_handle, |inner_handle| {

				if let Err(_) = tx.send( inner_handle ) {
					panic!("Unable to send browser handle back")
				}
			} );
		}).await?;

		Ok( BrowserThreaded::new( rx.await.unwrap() ) )
	}

	fn _build<H>( self, app: Application, on_created: H ) where
		H: FnOnce( BrowserHandle )
	{
		match self {
			Self { parent,
				   source,
				   title,
				   width,
				   height,
				   handler,
				   borders,
				   minimizable,
				   resizable,
				   dev_tools
			} => {

				// Parent
				let parent_handle = match parent {
					None => ptr::null(),
					Some( p ) => p.ffi_handle
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
					BrowserUserData {
						handler: match handler {
							Some(f) => f,
							None => Box::new(|_,_,_| {})
						}
					}
				) );
				let callback_data: *mut Box<dyn FnOnce( BrowserHandle )> = Box::into_raw( Box::new( Box::new(on_created ) ) );

				let window_options = bw_WindowOptions {
					minimizable: minimizable,
					resizable: resizable,
					closable: true,
					borders: borders
				};
				let other_options = bw_BrowserWindowOptions {
					dev_tools: dev_tools
				};

				unsafe { bw_BrowserWindow_new(
					app.handle.ffi_handle,
					parent_handle,
					csource,
					title_ptr,
					w,
					h,
					&window_options,
					&other_options,
					ffi_window_invoke_handler,
					user_data as _,
					ffi_browser_window_created_callback,
					callback_data as _
				) };
			}
		}
	}
}

/// The data that is passed to the C FFI handler function
struct BrowserUserData {
	handler: Box<dyn FnMut( BrowserHandle, &str, &[&str])>
}

/// The data that is passed to the creation callback function
//type BrowserCreationCallbackData<'a> = Box<dyn FnOnce( BrowserHandle ) + 'a>;



/// Takes the arguments received from the C FFI handler callback, and converts it to a vector of strings
fn args_to_vec<'a>( args: *const bw_CStrSlice, args_count: usize ) -> Vec<&'a str> {

	let mut vec = Vec::<&str>::with_capacity( args_count );

	for i in 0..args_count {
		let str_ref: &str = unsafe { (*args.offset(i as _)).into() };

		vec.push( str_ref );
	}

	vec
}

/// This external C function will be invoked by the underlying implementation browser-window when it is invoked in JavaScript
extern "C" fn ffi_window_invoke_handler( inner_handle: *mut bw_BrowserWindow, _command: bw_CStrSlice, _args: *const bw_CStrSlice, args_count: usize ) {

	unsafe {
		let data_ptr: *mut BrowserUserData = mem::transmute( bw_BrowserWindow_getUserData( inner_handle ) );
		let data: &mut BrowserUserData = &mut *data_ptr;

		match data {
			BrowserUserData{ handler } => {
				let outer_handle = BrowserHandle::new( inner_handle );

				let args = args_to_vec( _args, args_count );

				handler( outer_handle, _command.into(), &*args );
			}
		}
	}
}

// This external C function will be given as the callback to the bw_BrowserWindow_new function, to be invoked when the browser window has been created
extern "C" fn ffi_browser_window_created_callback( inner_handle: *mut bw_BrowserWindow, data: *mut c_void ) {

	unsafe {
		let data_ptr: *mut Box<dyn FnOnce( BrowserHandle )> = mem::transmute( data );
		let data = Box::from_raw( data_ptr );

		let outer_handle = BrowserHandle::new( inner_handle );

		data( outer_handle )
	}
}
