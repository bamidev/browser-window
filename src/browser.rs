//! This module contains all browser related handles and stuff.
//!
//! Keep in mind that `BrowserWindow` exposes the same methods as
//! `BrowserWindowHandle` and `WindowHandle` does. The methods of
//! `BrowserWindowHandle` are displayed correctly at the page of
//! `BrowserWindow`, but the methods of `WindowHandle` are not displayed.
//! Be sure to check them out [here](../window/struct.WindowHandle.html).

use std::{borrow::Cow, ops::Deref};

use futures_channel::oneshot;
#[cfg(feature = "threadsafe")]
use unsafe_send_sync::UnsafeSend;

#[cfg(feature = "threadsafe")]
use crate::delegate::*;
use crate::{
	application::*,
	core::{
		browser_window::{
			BrowserWindowEventExt, BrowserWindowExt, BrowserWindowImpl, JsEvaluationError,
		},
		window::WindowExt,
	},
	decl_browser_event, decl_event,
	event::EventHandler,
	prelude::*,
	rc::Rc,
	window::*,
};

mod builder;

pub use builder::{BrowserWindowBuilder, Source};


/// The future that dispatches a closure on the GUI thread.
#[cfg(feature = "threadsafe")]
pub type BrowserDelegateFuture<'a, R> = DelegateFuture<'a, BrowserWindowHandle, R>;

/// An owned browser window handle.
/// When this handle goes out of scope, its resources will get scheduled for
/// cleanup. The resources will only ever be cleaned up whenever both this
/// handle has gone out of scope, and when the window has actually been closed
/// by the user. If the window has been closed by the user but this handle still
/// exists, the window is actually just been closed. It can be reshown by
/// calling `show` on this handle.
pub struct BrowserWindowOwner(pub(super) BrowserWindowHandle);
pub struct BrowserWindow(pub(super) Rc<BrowserWindowOwner>);
#[cfg(feature = "threadsafe")]
pub struct BrowserWindowThreaded(BrowserWindow);
#[cfg(feature = "threadsafe")]
unsafe impl Sync for BrowserWindowThreaded {}

/// **Note:** Only available with feature `threadsafe` enabled.
///
/// A thread-safe handle to a browser window.
/// It allows you to dispatch code to the GUI thread and obtain manipulate the
/// browser window from any thread. To do this, you will need to use the
/// functions `dispatch`, `dispatch_async`, `delegate` and `delegate_async`.
///
/// # Example
///
/// This example fetches a value from within JavaScript:
/// ```
/// use browser_window::{application::*, browser::*};
///
/// async fn get_cookies(app: ApplicationHandleThreaded) -> String {
/// 	let mut builder =
/// 		BrowserWindowBuilder::new(Source::Url("https://www.duckduckgo.com/".into()));
/// 	builder.title("test");
///
/// 	let bw: BrowserWindowThreaded = builder.build_threaded(app).await.unwrap();
///
/// 	// Waits for `eval_js` to give back its result from the GUI thread
/// 	let result = bw
/// 		.delegate_async(|handle| async move {
/// 			// Execute `eval_js` on the GUI thread
/// 			handle.eval_js("document.cookies").await
/// 		})
/// 		.await
/// 		.unwrap();
///
/// 	result.unwrap()
/// }
/// ```

pub type BrowserWindowEventHandler<A> = EventHandler<BrowserWindowHandle, BrowserWindow, A>;

/// This is a handle to an existing browser window.
pub struct BrowserWindowHandle {
	pub(super) inner: BrowserWindowImpl,
	app: ApplicationHandle,
	window: WindowHandle,
}

pub struct MessageEventArgs<'a> {
	pub cmd: &'a str,
	pub args: Vec<JsValue>,
}

pub trait HasBrowserWindowHandle: HasWindowHandle {
	fn browser_handle(&self) -> &BrowserWindowHandle;
}


decl_browser_event!(AddressChangedEvent);
decl_browser_event!(AuthCredentialsEvent);
decl_browser_event!(CertificateErrorEvent);
decl_browser_event!(ConsoleMessageEvent);
decl_browser_event!(DownloadProgressEvent);
decl_browser_event!(DownloadStartedEvent);
decl_browser_event!(FaviconChangedEvent);
decl_browser_event!(FileDialogEvent);
decl_browser_event!(FullscreenModeChangedEvent);
decl_browser_event!(KeyPressEvent);
decl_browser_event!(KeyPressedEvent);
decl_browser_event!(LoadingProgressChangedEvent);
decl_browser_event!(MessageEvent);
decl_browser_event!(NavigationEndEvent);
decl_browser_event!(NavigationStartEvent);
decl_browser_event!(PageTitleChangedEvent);
decl_browser_event!(ScrollOffsetChangedEvent);
decl_browser_event!(SelectClientCertificateEvent);
decl_browser_event!(StartDraggingEvent);
decl_browser_event!(StatusMessageEvent);
decl_browser_event!(TooltipEvent);
decl_browser_event!(TextSelectionChangedEvent);


impl BrowserWindow {
	/// Whenver the address URI changes
	pub fn on_address_changed(&self) -> AddressChangedEvent {
		self.0.0.inner.on_address_changed(Rc::downgrade(&self.0))
	}

	/// When a console message is printend.
	pub fn on_console_message(&self) -> ConsoleMessageEvent {
		self.0.0.inner.on_console_message(Rc::downgrade(&self.0))
	}

	/// Whenever the browser goes into or out of full screen mode.
	pub fn on_fullscreen_mode_changed(&self) -> FullscreenModeChangedEvent {
		self.0
			.0
			.inner
			.on_fullscreen_mode_changed(Rc::downgrade(&self.0))
	}

	/// Loading progress updates
	pub fn on_loading_progress_changed(&self) -> LoadingProgressChangedEvent {
		self.0
			.0
			.inner
			.on_loading_progress_changed(Rc::downgrade(&self.0))
	}

	/// The event that will fire whenever `invoke_extern` is called with JS on
	/// the client side.
	pub fn on_message(&self) -> MessageEvent { self.0.0.inner.on_message(Rc::downgrade(&self.0)) }

	/// Whenever navigation has finished and the page has loaded.
	pub fn on_navigation_end(&self) -> NavigationEndEvent {
		self.0.0.inner.on_navigation_end(Rc::downgrade(&self.0))
	}

	/// Whenever navigation to a new link happens.
	pub fn on_navigation_start(&self) -> NavigationStartEvent {
		self.0.0.inner.on_navigation_start(Rc::downgrade(&self.0))
	}

	/// Whenver the page title changes.
	pub fn on_page_title_changed(&self) -> PageTitleChangedEvent {
		self.0.0.inner.on_page_title_changed(Rc::downgrade(&self.0))
	}

	pub fn on_status_message(&self) -> StatusMessageEvent {
		self.0.0.inner.on_status_message(Rc::downgrade(&self.0))
	}

	/// Whenever the browser is about to show a tooltip
	pub fn on_tooltip(&self) -> TooltipEvent { self.0.0.inner.on_tooltip(Rc::downgrade(&self.0)) }

	/// Not implemented yet.
	pub fn on_auth_credentials(&self) -> AuthCredentialsEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_certificate_error(&self) -> CertificateErrorEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_download_progress(&self) -> DownloadProgressEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_download_started(&self) -> DownloadStartedEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_favicon_changed(&self) -> FaviconChangedEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_file_dialog(&self) -> FileDialogEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_key_press(&self) -> KeyPressEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_key_pressed(&self) -> KeyPressedEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_scroll_offset_changed(&self) -> ScrollOffsetChangedEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_select_client_certificate(&self) -> SelectClientCertificateEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_start_dragging(&self) -> StartDraggingEvent {
		unimplemented!();
	}

	/// Not implemented yet.
	pub fn on_text_selection_changed(&self) -> TextSelectionChangedEvent {
		unimplemented!();
	}
}

impl Deref for BrowserWindow {
	type Target = BrowserWindowHandle;

	fn deref(&self) -> &Self::Target { &self.0.0 }
}

// Core browser window functions
impl BrowserWindowHandle {
	/// Returns the application handle associated with this browser window.
	pub fn app(&self) -> ApplicationHandle { ApplicationHandle::new(self.inner.window().app()) }

	pub fn close(self) {
		// The window isn't actually destroyed until the reference count of the owner
		// reaches 0.
		self.inner.window().hide();
	}

	/// Executes the given javascript code and returns the output as a string.
	/// If you don't need the result, see `exec_js`.
	///
	/// There may be some discrepancies in what JS values are being returned for
	/// the same code in different browser engines, or how accurate they are.
	/// For example, Edge WebView2 doesn't return `JsValue::Undefined`, it uses
	/// `JsValue::Null` instead.
	pub async fn eval_js(&self, js: &str) -> Result<JsValue, JsEvaluationError> {
		//
		let (tx, rx) = oneshot::channel();

		self._eval_js(js, |_, result| {
			// The callback of `_eval_js` is not necessarily called from our GUI thread, so
			// this is a quickfix to make that safe. Otherwise, the `panic` may not be
			// propegated correctly! FIXME: Call this result callback closure form the GUI
			// thread from within the CEF implementation.
			if let Err(_) = tx.send(result) {
				panic!("Unable to send JavaScript result back")
			}
		});

		rx.await.unwrap()
	}

	/// Executes the given JavaScript code, and provides the output via a
	/// callback.
	///
	/// # Arguments
	/// * `on_complete` - The closure that will be called when the output is
	///   ready.
	fn _eval_js<'a, H>(&self, js: &str, on_complete: H)
	where
		H: FnOnce(&BrowserWindowHandle, Result<JsValue, JsEvaluationError>) + 'a,
	{
		let data_ptr: *mut H = Box::into_raw(Box::new(on_complete));

		self.inner
			.eval_js(js.into(), eval_js_callback::<H>, data_ptr as _);
	}

	/// Executes the given javascript code without waiting on it to finish.
	pub fn exec_js(&self, js: &str) { self._eval_js(js, |_, _| {}); }

	/// Causes the browser to navigate to the given url.
	pub fn navigate(&self, url: &str) { self.inner.navigate(url) }

	pub fn url<'a>(&'a self) -> Cow<'a, str> { self.inner.url() }

	pub fn window(&self) -> &WindowHandle { &self.window }
}

// Functions to reach the UI thread
#[cfg(feature = "threadsafe")]
impl BrowserWindowThreaded {
	/// The thread-safe application handle associated with this browser window.
	pub fn app(&self) -> ApplicationHandleThreaded {
		ApplicationHandleThreaded::from_core_handle(self.handle.inner.window().app())
	}

	/// Closes the browser.
	pub fn close(self) -> bool { self.dispatch(|bw| bw.close()) }

	/// Executes the given closure within the GUI thread, and return the value
	/// that the closure returned. Also see `ApplicationThreaded::delegate`.
	///
	/// The function signature is practically the same as:
	/// ```ignore
	/// pub async fn delegate<'a,F,R>( &self, func: F ) -> Result<R, DelegateError> where
	/// 	F: FnOnce( BrowserWindowHandle ) -> R + Send + 'a,
	/// 	R: Send { /*...*/ }
	/// ```
	pub fn delegate<'a, F, R>(&self, func: F) -> BrowserDelegateFuture<'a, R>
	where
		F: FnOnce(&BrowserWindowHandle) -> R + Send + 'a,
		R: Send,
	{
		BrowserDelegateFuture::new(self.handle.clone(), func)
	}

	/// Executes the given async closure `func` on the GUI thread, and gives
	/// back the result when done.
	/// Also see `ApplicationThreaded::delegate_async`.
	pub fn delegate_async<'a, C, F, R>(&self, func: C) -> DelegateFutureFuture<'a, R>
	where
		C: FnOnce(&BrowserWindowHandle) -> F + Send + 'a,
		F: Future<Output = R>,
		R: Send + 'static,
	{
		let handle = self.handle.clone();
		DelegateFutureFuture::new(self.app().handle.clone(), async move {
			func(handle.into()).await
		})
	}

	/// Executes the given future on the GUI thread, and gives back the result
	/// when done. Also see `ApplicationThreaded::delegate_future`.
	pub fn delegate_future<'a, F, R>(&self, fut: F) -> DelegateFutureFuture<'a, R>
	where
		F: Future<Output = R> + Send + 'a,
		R: Send + 'static,
	{
		let handle = self.handle.clone();
		DelegateFutureFuture::new(self.app().handle.clone(), fut)
	}

	/// Executes the given close on the GUI thread.
	/// See also `Application::dispatch`.
	pub fn dispatch<'a, F>(&self, func: F) -> bool
	where
		F: FnOnce(&BrowserWindowHandle) + Send + 'a,
	{
		let handle = UnsafeSend::new(self.handle.clone());

		// FIXME: It is more efficient to reimplement this for the browser window
		self.app().dispatch(move |_| {
			func(handle.i);
		})
	}

	/// Executes the given closure on the GUI thread.
	/// See also `Application::dispatch`.
	pub fn dispatch_async<'a, C, F>(&self, func: C) -> bool
	where
		C: FnOnce(&BrowserWindowHandle) -> F + Send + 'a,
		F: Future<Output = ()> + 'static,
	{
		let handle = UnsafeSend::new(self.handle.clone());

		self.app().dispatch(move |a| {
			a.spawn(func(handle.i));
		})
	}

	fn new(handle: BrowserWindowHandle) -> Self { Self { handle } }
}

impl BrowserWindowHandle {
	pub(crate) fn new(inner_handle: BrowserWindowImpl) -> Self {
		Self {
			app: ApplicationHandle::new(inner_handle.window().app()),
			window: WindowHandle::new(inner_handle.window()),
			inner: inner_handle,
		}
	}
}

impl Deref for BrowserWindowHandle {
	type Target = WindowHandle;

	fn deref(&self) -> &Self::Target { &self.window }
}

impl HasAppHandle for BrowserWindowHandle {
	fn app_handle(&self) -> &ApplicationHandle { &self.app }
}

impl BrowserWindowOwner {
	fn cleanup(handle: &BrowserWindowHandle) {
		handle.inner.free();
		handle.inner.window().free();
	}
}

impl Deref for BrowserWindowOwner {
	type Target = BrowserWindowHandle;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl Drop for BrowserWindowOwner {
	fn drop(&mut self) {
		#[cfg(not(feature = "threadsafe"))]
		Self::cleanup(&self.0);
		#[cfg(feature = "threadsafe")]
		self.app().dispatch(|bw| {
			Self::cleanup(bw);
		});
	}
}


fn eval_js_callback<H>(
	_handle: BrowserWindowImpl, cb_data: *mut (), result: Result<JsValue, JsEvaluationError>,
) where
	H: FnOnce(&BrowserWindowHandle, Result<JsValue, JsEvaluationError>),
{
	let data_ptr = cb_data as *mut H;
	let data = unsafe { Box::from_raw(data_ptr) };

	let handle = BrowserWindowHandle::new(_handle);

	(*data)(&handle, result);
}
