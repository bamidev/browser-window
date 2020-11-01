use browser_window_ffi::*;

use boxfnonce::SendBoxFnOnce;
use std::ffi::*;
use tokio::sync::oneshot;

use crate::application::{Application, ApplicationAsync, ApplicationHandle};
use crate::browser_window::{BrowserWindow, BrowserWindowHandle, BrowserWindowInner, BrowserWindowAsync};

use std::{
	mem,
	ops::Deref,
	ptr,
	rc::Rc,
	sync::Arc
};



/// The type of content to display in a browser window
pub enum Source {
	Url( String ),
	Html( String )
}

/// Used to create a BrowserWindow instance.
pub struct BrowserWindowBuilder {

	parent: Option<BrowserWindowHandle>,
	source: Source,
	title: Option<String>,
	width: Option<u32>,
	height: Option<u32>,
	handler: Option<Box<dyn FnMut(BrowserWindowHandle, &str, &[&str]) + Send>>
}



impl BrowserWindowBuilder {

	/// Configure a closure that can be invoked from within JavaScript.
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

	/// Creates the browser window.
	///
	/// # Arguments
	/// * `app` - An application handle that this browser window can spawn into
	/// * `on_created` - A callback closure that will be invoked when the browser window is created and ready.
	pub fn spawn<H>( self, app: &Application, on_created: H ) where
		H: FnOnce( BrowserWindow ) + Send + 'static
	{
		let app_handle = (*app.inner).clone();

		self._spawn( app_handle.clone(), move |inner_handle| {
			let bw = BrowserWindow {
				inner: Rc::new( BrowserWindowInner {
					app: app_handle,
					handle: inner_handle
				} )
			};

			on_created( bw );
		} );
	}

	/// Same as spawn, but asynchronous.
	/// Instead of providing a callback, the handle is simply returned.
	///
	/// # Arguments
	/// * `app` - An async application handle.
	pub async fn spawn_async( self, app: &ApplicationAsync ) -> BrowserWindowAsync {
		let (tx, rx) = oneshot::channel::<BrowserWindowHandle>();

		let app_handle = (*app.inner).clone();

		// We need to dispatch the spawning to the GUI thread because only there we can call GUI functionality
		app.dispatch(|app| {
			self._spawn(app, move |inner_handle| {
				let _ = tx.send( inner_handle );
			} );
		}).await;

		let inner_handle = rx.await.unwrap();

		BrowserWindowAsync {
			inner: Arc::new( BrowserWindowInner {
				app: app_handle,
				handle: inner_handle
			} )
		}
	}

	fn _spawn<H: 'static>( self, app: ApplicationHandle, on_created: H ) where
		H: FnOnce( BrowserWindowHandle ) + Send + 'static
	{
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
					BrowserWindowUserData {
						handler: match handler {
							None => Box::new(|_,_,_| {}),
							Some(f) => Box::new(f)
						}
					}
				) );
				let callback_data = Box::into_raw( Box::new(
					BrowserWindowCreationCallbackData::from( on_created )
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

				unsafe { bw_BrowserWindow_new(
					app._ffi_handle.clone(),
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
struct BrowserWindowUserData {
	handler: Box<dyn FnMut( BrowserWindowHandle, &str, &[&str])>
}

/// The data that is passed to the creation callback function
type BrowserWindowCreationCallbackData = SendBoxFnOnce<'static, ( BrowserWindowHandle, )>;



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
		let data_ptr: *mut BrowserWindowUserData = mem::transmute( bw_BrowserWindow_getUserData( inner_handle ) );
		let data: &mut BrowserWindowUserData = &mut *data_ptr;

		match data {
			BrowserWindowUserData{ handler } => {
				let outer_handle = BrowserWindowHandle::from_ptr( inner_handle );

				let args = args_to_vec( _args, args_count );

				handler( outer_handle, _command.into(), &*args );
			}
		}
	}
}

// This external C function will be given as the callback to the bw_BrowserWindow_new function, to be invoked when the browser window has been created
extern "C" fn ffi_browser_window_created_callback( inner_handle: *mut bw_BrowserWindow, data: *mut c_void ) {

	unsafe {
		let data_ptr: *mut BrowserWindowCreationCallbackData = mem::transmute( data );
		let data = Box::from_raw( data_ptr );

		let outer_handle = BrowserWindowHandle::from_ptr( inner_handle );

		data.call( outer_handle );
	}
}
