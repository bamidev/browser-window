use browser_window_core::*;
use browser_window_core::browser_window::*;
use browser_window_core::window::*;

use crate::application::{ApplicationHandle};
use crate::browser::*;
use crate::window::WindowBuilder;

use std::{
	ops::DerefMut,
	path::PathBuf,
	pin::Pin,
	vec::Vec
};



/// The type of content to display in a browser window
pub enum Source {
	/// Displays the given HTML code in the browser.
	Html( String ),
	/// Displays the local file for the given path.
	File( PathBuf ),
	/// Displays the given URL in the browser.
	Url( String )
}

#[cfg(not(feature = "threadsafe"))]
type BrowserJsInvocationHandler = Box<dyn FnMut(BrowserWindowHandle, String, Vec<String>) -> Pin<Box<dyn Future<Output=()>>>>;
#[cfg(feature = "threadsafe")]
type BrowserJsInvocationHandler = Box<dyn FnMut(BrowserWindowHandle, String, Vec<String>) -> Pin<Box<dyn Future<Output=()>>> + Send>;

/// The data that is passed to the C FFI handler function
struct BrowserUserData {
	handler: BrowserJsInvocationHandler
}

/// Used to create a `BrowserWindow` or `BrowserWindowThreaded` instance.
/// 
/// # Warning
/// `BrowserWindowBuilder` dereferences to `WindowBuilder`, so that you can modify anything related to the window as well.
/// This comes with a catch though.
/// If you call any functions from `WindowBuilder`, they return a reference to `WindowBuilder`.
/// So the following doesn't work:
/// ```ignore
/// let bwb = BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com".into()) )
/// 	.title("DuckDuckGo")
/// 	.dev_tools(true);
/// ```
/// This is because the call to `title` returns a reference to `WindowBuilder`, which doesn't have a method `dev_tools`.
/// 
/// The following is also wrong:
/// ```ignore
/// let bw = BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com".into()) )
/// 	.dev_tools(true)
/// 	.title("DuckDuckGo")
/// 	.build();
/// ```
/// This is because _Browser Window_ currently does not support creating windows that don't have browsers in them.
/// `build` is a method of `BrowserWindowBuilder`, yet `title` returns a reference to `WindowBuilder`, which has no `build` method.
/// 
/// The solution is to do this:
/// ```
/// let mut bwb = BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com".into()) )
/// 	.dev_tools(true);
/// bwb.title("DuckDuckGo");
/// let bw = bwb.build();
/// ```
pub struct BrowserWindowBuilder {

	dev_tools: bool,
	handler: Option<BrowserJsInvocationHandler>,
	source: Source,
	window: WindowBuilder
}



impl BrowserWindowBuilder {

	/// Configure a closure that can be invoked from within JavaScript.
	/// The closure's second parameter specifies a command name.
	/// The closure's third parameter specifies an array of string arguments.
	#[cfg(not(feature = "threadsafe"))]
	pub fn async_handler<H,F>( &mut self, mut handler: H ) -> &mut Self where
		H: FnMut(BrowserWindowHandle, String, Vec<String>) -> F + 'static,
		F: Future<Output=()> + 'static
	{
		self.handler = Some( Box::new(
			move |handle, cmd, args| Box::pin(handler( handle, cmd, args ) )
		) );
		self
	}

	/// Configure a closure that can be invoked from within JavaScript.
	/// The closure's second parameter specifies a command name.
	/// The closure's third parameter specifies an array of string arguments.
	#[cfg(feature = "threadsafe")]
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
	#[cfg(not(feature = "threadsafe"))]
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
	#[cfg(feature = "threadsafe")]
	pub async fn build( self, app: ApplicationHandleThreaded ) -> Result<BrowserWindowThreaded, DelegateError> {

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
					None => WindowImpl::default(),
					Some( p ) => p.i.inner
				};

				// Source
				let mut _url: PathBuf = "file:///".into();	// Stays here so that the reference to it that gets passed to C stays valid for the function call to `bw_BrowserWindow_new`.
				let source = match &source {	// Use a reference, we want source to live until the end of the function because bw_BrowserWindowSource holds a reference to its internal string.
					Source::File( path ) => {
						_url.push( path );

						browser_window::Source {
							data: _url.to_str().unwrap().into(),
							is_html: false
						}
					},
					Source::Html( html ) => { browser_window::Source {
						data: html.as_str().into(),
						is_html: true
					} },
					Source::Url( url ) => { browser_window::Source {
						data: url.as_str().into(),
						is_html: false
					} }
				};

				// Title
				let title = match window.title.as_ref() {
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
				let window_options = WindowOptions {
					borders: window.borders,
					minimizable: window.minimizable,
					resizable: window.resizable
				};
				let other_options = BrowserWindowOptions {
					dev_tools,
					resource_path: "".into()
				};

				BrowserWindowImpl::new(
					app.inner,
					parent_handle,
					source,
					title,
					window.width,
					window.height,
					&window_options,
					&other_options,
					browser_window_invoke_handler,
					user_data as _,
					browser_window_created_callback,
					callback_data as _
				);
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

/// The data that is passed to the creation callback function
//type BrowserCreationCallbackData<'a> = Box<dyn FnOnce( BrowserHandle ) + 'a>;



/// Takes the arguments received from the C FFI handler callback, and converts it to a vector of strings
/*fn args_to_vec( args: *const cbw_CStrSlice, args_count: usize ) -> Vec<String> {

	let mut vec = Vec::with_capacity( args_count );

	for i in 0..args_count {
		let str_arg: String = unsafe { *args.offset(i as _) }.into();

		vec.push( str_arg );
	}

	vec
}*/

/// This external C function will be invoked by the underlying implementation browser-window when it is invoked in JavaScript
/*unsafe extern "C" fn ffi_window_invoke_handler( inner_handle: *mut bw_BrowserWindow, _command: bw_CStrSlice, _args: *mut bw_CStrSlice, args_count: u64 ) {

	let data_ptr: *mut BrowserUserData = mem::transmute( bw_BrowserWindow_getUserData( inner_handle ) );
	let data: &mut BrowserUserData = &mut *data_ptr;

	match data {
		BrowserUserData{ handler } => {
			let outer_handle = BrowserWindowHandle::new( inner_handle );

			let args = args_to_vec( _args, args_count as _ );

			let future = handler( outer_handle, _command.into(), args );
			outer_handle.app().spawn( future );
		}
	}
}*/

unsafe fn browser_window_invoke_handler( inner_handle: BrowserWindowImpl, cmd: &str, args: Vec<String> ) {
	
	let data_ptr: *mut BrowserUserData = inner_handle.user_data() as _;
	let data = &mut *data_ptr;

	match data {
		BrowserUserData{ handler } => {
			let outer_handle = BrowserWindowHandle::new( inner_handle );

			let future = handler( outer_handle, cmd.into(), args );
			outer_handle.app().spawn( future );
		}
	}
}

// This external C function will be given as the callback to the bw_BrowserWindow_new function, to be invoked when the browser window has been created
/*unsafe extern "C" fn ffi_browser_window_created_callback( inner_handle: *mut bw_BrowserWindow, data: *mut c_void ) {

	let data_ptr: *mut Box<dyn FnOnce( BrowserWindowHandle )> = mem::transmute( data );
	let data = Box::from_raw( data_ptr );

	let outer_handle = BrowserWindowHandle::new( inner_handle );

	data( outer_handle )
}*/

unsafe fn browser_window_created_callback( inner_handle: BrowserWindowImpl, data: *mut () ) {

	let data_ptr = data as *mut Box<dyn FnOnce( BrowserWindowHandle )>;
	let func = Box::from_raw( data_ptr );

	let outer_handle = BrowserWindowHandle::new( inner_handle );

	func( outer_handle )
}