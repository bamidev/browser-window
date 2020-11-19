use browser_window_ffi::*;

use std::ffi::*;

use crate::application::{ApplicationHandle, ApplicationHandleThreaded};
use crate::browser::*;
use crate::window::WindowBuilder;

use std::{
	ops::DerefMut,
	mem,
	path::PathBuf,
	pin::Pin,
	ptr,
	vec::Vec
};

use unsafe_send_sync::UnsafeSend;


/// The type of content to display in a browser window
pub enum Source {
	/// Displays the given HTML code in the browser.
	Html( String ),
	/// Displays the local file for the given path.
	File( PathBuf ),
	/// Displays the given URL in the browser.
	Url( String )
}

/// Used to create a `Browser` or `BrowserThreaded` instance.
pub struct BrowserWindowBuilder {

	dev_tools: bool,
	handler: Option<Box<dyn FnMut(BrowserWindowHandle, String, Vec<String>) -> Pin<Box<dyn Future<Output=()>>> + Send>>,
	source: Source,
	window: WindowBuilder
}



impl BrowserWindowBuilder {

	/// Configure a closure that can be invoked from within JavaScript.
	/// The closure's second parameter specifies a command name.
	/// The closure's third parameter specifies an array of string arguments.
	pub fn async_handler<H,F>( &mut self, mut handler: H ) -> &mut Self where
		H: FnMut(BrowserWindowHandle, String, Vec<String>) -> F + Send + 'static,
		F: Future<Output=()> + 'static
	{
		self.handler = Some( Box::new(
			move |handle, cmd, args| Box::pin(handler( handle, cmd, args ) )
		) );
		self
	}

	/// Sets whether or not an extra window with developer tools will be opened together with this browser.
	/// When in debug mode the default is `true`.
	/// When in release mode the default is `false`.
	pub fn dev_tools( &mut self, enabled: bool ) -> &mut Self {
		self.dev_tools = enabled;	self
	}

	/*pub fn handler<H>( &mut self, mut handler: H ) -> &Self where
		H: FnMut(BrowserWindowHandle, String, Vec<String>) + Send + 'static
	{
		self.handler = Some( Box::new( move |handle, cmd, args| Box::pin( async {
			handler( handle, cmd, args );
		} ) ) );
		self
	}*/

	/// Creates an instance of a browser window builder.
	///
	/// # Arguments
	/// * `source` - The content that will be displayed in the browser window.
	pub fn new( source: Source ) -> Self {
		Self {
			dev_tools: cfg!(debug_assertions),
			source,
			handler: None,
			window: WindowBuilder::new()
		}
	}

	/// Creates the browser window.
	///
	/// # Arguments
	/// * `app` - An application handle that this browser window can spawn into
	pub async fn build( self, app: ApplicationHandle ) -> BrowserWindow
	{
		let (tx, rx) = oneshot::channel::<BrowserWindowHandle>();

		self._build( app, move |handle| {

			if let Err(_) = tx.send( handle ) {
				panic!("Unable to send browser handle back")
			}
		} );

		BrowserWindow::new( rx.await.unwrap() )
	}

	/// Same as build, but gives back a browser handle that is thread-safe.
	///
	/// # Arguments
	/// * `app` - An thread-safe application handle.
	pub async fn build_threaded( self, app: ApplicationHandleThreaded ) -> Result<BrowserWindowThreaded, DelegateError> {

		let (tx, rx) = oneshot::channel::<UnsafeSend<BrowserWindowHandle>>();

		// We need to dispatch the spawning of the browser to the GUI thread
		app.delegate(|app_handle| {

			self._build(app_handle, |inner_handle| {

				if let Err(_) = tx.send( UnsafeSend::new( inner_handle ) ) {
					panic!("Unable to send browser handle back")
				}
			} );
		}).await?;

		Ok( BrowserWindowThreaded::new( rx.await.unwrap().i ) )
	}

	fn _build<H>( self, app: ApplicationHandle, on_created: H ) where
		H: FnOnce( BrowserWindowHandle )
	{
		match self {
			Self {
				source,
				handler,
				dev_tools,
				window
			} => {

				// Parent
				let parent_handle = match window.parent {
					None => ptr::null(),
					Some( p ) => p.i.ffi_handle
				};

				// Source
				let mut _url: PathBuf = "file:///".into();	// Stays here so that the reference to it that gets passed to C stays valid for the function call to `bw_BrowserWindow_new`.
				let csource = match &source {	// Use a reference, we want source to live until the end of the function because bw_BrowserWindowSource holds a reference to its internal string.
					Source::File( path ) => {
						_url.push( path );

						bw_BrowserWindowSource {
							data: _url.to_str().unwrap().into(),
							is_html: false
						}
					},
					Source::Html( html ) => { bw_BrowserWindowSource {
						data: html.as_str().into(),
						is_html: true
					} },
					Source::Url( url ) => { bw_BrowserWindowSource {
						data: url.as_str().into(),
						is_html: false
					} }
				};

				// Title
				let title_ptr = match window.title.as_ref() {
					None => "Browser Window".into(),
					Some( t ) => t.as_str().into()
				};

				// Handler callback data
				let user_data = Box::into_raw( Box::new(
					BrowserUserData {
						handler: match handler {
							Some(f) => f,
							None => Box::new(|_,_,_| Box::pin(async {}))
						}
					}
				) );
				let callback_data: *mut Box<dyn FnOnce( BrowserWindowHandle )> = Box::into_raw( Box::new( Box::new(on_created ) ) );

				// Convert options to FFI structs
				let window_options = bw_WindowOptions {
					borders: window.borders,
					closable: true,
					minimizable: window.minimizable,
					opacity: window.opacity,
					resizable: window.resizable
				};
				let other_options = bw_BrowserWindowOptions {
					dev_tools,
					resource_path: "".into()
				};

				unsafe { bw_BrowserWindow_new(
					app.ffi_handle,
					parent_handle,
					csource,
					title_ptr,
					window.width,
					window.height,
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

impl Deref for BrowserWindowBuilder {
	type Target = WindowBuilder;

	fn deref( &self ) -> &Self::Target {
		&self.window
	}
}

impl DerefMut for BrowserWindowBuilder {

	fn deref_mut( &mut self ) -> &mut Self::Target {
		&mut self.window
	}
}

/// The data that is passed to the C FFI handler function
struct BrowserUserData {
	handler: Box<dyn FnMut( BrowserWindowHandle, String, Vec<String>) -> Pin<Box<dyn Future<Output=()>>>>
}

/// The data that is passed to the creation callback function
//type BrowserCreationCallbackData<'a> = Box<dyn FnOnce( BrowserHandle ) + 'a>;



/// Takes the arguments received from the C FFI handler callback, and converts it to a vector of strings
fn args_to_vec( args: *const bw_CStrSlice, args_count: usize ) -> Vec<String> {

	let mut vec = Vec::with_capacity( args_count );

	for i in 0..args_count {
		let str_arg: String = unsafe { *args.offset(i as _) }.into();

		vec.push( str_arg );
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
				let outer_handle = BrowserWindowHandle::new( inner_handle );

				let args = args_to_vec( _args, args_count );

				let future = handler( outer_handle, _command.into(), args );
				outer_handle.app().spawn( future );
			}
		}
	}
}

// This external C function will be given as the callback to the bw_BrowserWindow_new function, to be invoked when the browser window has been created
extern "C" fn ffi_browser_window_created_callback( inner_handle: *mut bw_BrowserWindow, data: *mut c_void ) {

	unsafe {
		let data_ptr: *mut Box<dyn FnOnce( BrowserWindowHandle )> = mem::transmute( data );
		let data = Box::from_raw( data_ptr );

		let outer_handle = BrowserWindowHandle::new( inner_handle );

		data( outer_handle )
	}
}
