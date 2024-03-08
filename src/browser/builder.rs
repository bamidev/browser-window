use std::{ops::DerefMut, path::PathBuf};

#[cfg(feature = "threadsafe")]
use unsafe_send_sync::UnsafeSend;

use crate::{
	application::ApplicationHandle,
	browser::*,
	core::{browser_window::*, window::*},
	rc::Rc,
	window::WindowBuilder,
};

/// The type of content to display in a browser window
#[derive(Clone)]
pub enum Source {
	/// Displays the given HTML code in the browser.
	Html(String),
	/// Displays the local file for the given path.
	File(PathBuf),
	/// Displays the given URL in the browser.
	Url(String),
}

/// The data that is passed to the C FFI handler function
struct BrowserUserData {
	_handle: Rc<BrowserWindowOwner>,
}

/// Used to create a [`BrowserWindow`] or [`BrowserWindowThreaded`] instance,
/// depending on whether or not you have feature `threadsafe` enabled.
///
/// ```ignore
/// let mut bwb = BrowserWindowBuilder::new(Source::Url("https://www.duckduckgo.com".into()))
/// bwb.dev_tools(true);
/// bwb.title("DuckDuckGo");
/// let bw = bwb.build( app );
/// ```
pub struct BrowserWindowBuilder {
	dev_tools: bool,
	source: Source,
	window: WindowBuilder,
}

impl BrowserWindowBuilder {
	/// Sets whether or not an extra window with developer tools will be opened
	/// together with this browser. When in debug mode the default is `true`.
	/// When in release mode the default is `false`.
	pub fn dev_tools(&mut self, enabled: bool) -> &mut Self {
		self.dev_tools = enabled;
		self
	}

	/// Creates an instance of a browser window builder.
	///
	/// # Arguments
	/// * `source` - The content that will be displayed in the browser window.
	pub fn new(source: Source) -> Self {
		Self {
			dev_tools: false,
			source,
			window: WindowBuilder::new(),
		}
	}

	/// Creates the browser window.
	///
	/// # Arguments
	/// * `app` - An application handle that this browser window can spawn into
	pub async fn build(self, app: ApplicationHandle) -> BrowserWindow {
		let (tx, rx) = oneshot::channel::<BrowserWindowHandle>();

		self._build(app, move |handle| {
			if let Err(_) = tx.send(handle) {
				panic!("Unable to send browser handle back")
			}
		});

		BrowserWindow(Self::prepare_handle(rx.await.unwrap()))
	}

	/// Creates the browser window.
	///
	/// Keep in mind that the description of this function is for when feature
	/// `threadsafe` is enabled. When it is not enabled, it looks like this:
	/// ```ignore
	/// pub async fn build( self, app: ApplicationHandle ) -> Result<BrowserWindowThreaded, DelegateError> { /* ... */ }
	/// ```
	///
	/// # Arguments
	/// * `app` - An (thread-safe) application handle.
	#[cfg(feature = "threadsafe")]
	pub async fn build_threaded(
		self, app: ApplicationHandle,
	) -> Result<BrowserWindowThreaded, DelegateError> {
		let (tx, rx) = oneshot::channel::<UnsafeSend<BrowserWindowHandle>>();

		// We need to dispatch the spawning of the browser to the GUI thread
		app.delegate(|app_handle| {
			self._build(app_handle, |inner_handle| {
				if let Err(_) = tx.send(UnsafeSend::new(inner_handle)) {
					panic!("Unable to send browser handle back")
				}
			});
		})
		.await?;

		Ok(BrowserWindowThreaded::new(Self::prepare_handle(rx.await.unwrap())))
	}

	fn prepare_handle(handle: BrowserWindowHandle) -> Rc<BrowserWindowOwner> {
		// Put a reference counted handle in the user data of the window, so that there exists 'ownership' for as long as the window actually lives.
		let owner = BrowserWindowOwner(handle);
		let rc_handle = Rc::new(owner);
		let user_data = Box::into_raw(Box::new(BrowserUserData {
			_handle: rc_handle.clone()
		}));
		rc_handle.0.window().0.set_user_data(user_data as _);

		rc_handle
	}

	fn _build<H>(self, app: ApplicationHandle, on_created: H)
	where
		H: FnOnce(BrowserWindowHandle),
	{
		match self {
			Self {
				source,
				dev_tools,
				window,
			} => {
				// Parent
				let parent_handle = match window.parent {
					None => WindowImpl::default(),
					Some(p) => p.i,
				};

				// Title
				let title = match window.title.as_ref() {
					None => "Browser Window".into(),
					Some(t) => t.as_str().into(),
				};

				let callback_data: *mut Box<dyn FnOnce(BrowserWindowHandle)> =
					Box::into_raw(Box::new(Box::new(on_created)));

				// Convert options to FFI structs
				let window_options = WindowOptions {
					borders: window.borders,
					minimizable: window.minimizable,
					resizable: window.resizable,
				};
				let other_options = BrowserWindowOptions {
					dev_tools: if dev_tools { 1 } else { 0 },
					resource_path: "".into(),
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
					browser_window_created_callback,
					callback_data as _,
				);
			}
		}
	}
}

impl Deref for BrowserWindowBuilder {
	type Target = WindowBuilder;

	fn deref(&self) -> &Self::Target { &self.window }
}

impl DerefMut for BrowserWindowBuilder {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.window }
}

fn browser_window_created_callback(inner_handle: BrowserWindowImpl, data: *mut ()) {
	let data_ptr = data as *mut Box<dyn FnOnce(&BrowserWindowHandle)>;
	let func = unsafe { Box::from_raw(data_ptr) };

	let rust_handle = BrowserWindowHandle::new(inner_handle);

	func(&rust_handle);
}
