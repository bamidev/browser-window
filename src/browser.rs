//! This module contains all browser related handles and stuff.
//!
//! Keep in mind that `BrowserWindow` exposes the same methods as
//! `BrowserWindowHandle` and `WindowHandle` does. The methods of
//! `BrowserWindowHandle` are displayed correctly at the page of
//! `BrowserWindow`, but the methods of `WindowHandle` are not displayed.
//! Be sure to check them out [here](../window/struct.WindowHandle.html).

use std::{borrow::Cow, future::Future, marker::PhantomData, ops::Deref, rc::Rc};

use futures_channel::oneshot;
#[cfg(feature = "threadsafe")]
use unsafe_send_sync::UnsafeSend;

#[cfg(feature = "threadsafe")]
use crate::delegate::*;
use crate::{
	application::*,
	core::{
		browser_window::{BrowserWindowExt, BrowserWindowImpl, JsEvaluationError},
		window::WindowExt,
	},
	prelude::*,
	window::*,
};

mod builder;

pub use builder::{BrowserWindowBuilder, Source};

/// The future that dispatches a closure on the GUI thread.
#[cfg(feature = "threadsafe")]
pub type BrowserDelegateFuture<'a, R> = DelegateFuture<'a, BrowserWindowHandle, R>;

/// An owned browser window handle.
/// When this handle goes out of scope, its resource get scheduled for cleanup.
// If the user closes the window, this handle remains valid.
// Also, if you lose this handle, window destruction and cleanup is only done
// when the user actually closes it. So you don't have to worry about lifetimes
// and/or propper destruction of the window either.
pub struct BrowserWindow {
	pub(super) handle: BrowserWindowHandle,
	_not_send: PhantomData<Rc<()>>,
}

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
#[cfg(feature = "threadsafe")]
pub struct BrowserWindowThreaded {
	pub(super) handle: BrowserWindowHandle,
}
#[cfg(feature = "threadsafe")]
unsafe impl Sync for BrowserWindowThreaded {}

/// This is a handle to an existing browser window.
#[derive(Clone)]
pub struct BrowserWindowHandle {
	pub(super) inner: BrowserWindowImpl,
	window: WindowHandle,
}

pub trait OwnedBrowserWindow: OwnedWindow {
	fn browser_handle(&self) -> BrowserWindowHandle;
}

impl BrowserWindow {
	fn new(handle: BrowserWindowHandle) -> Self {
		Self {
			handle,
			_not_send: PhantomData,
		}
	}
}

impl Deref for BrowserWindow {
	type Target = BrowserWindowHandle;

	fn deref(&self) -> &Self::Target { &self.handle }
}

impl Drop for BrowserWindow {
	fn drop(&mut self) { self.handle.inner.window().drop(); }
}

impl HasAppHandle for BrowserWindow {
	fn app_handle(&self) -> ApplicationHandle { self.handle.app_handle() }
}

impl OwnedWindow for BrowserWindow {
	fn window_handle(&self) -> WindowHandle { self.handle.window() }
}

impl OwnedBrowserWindow for BrowserWindow {
	fn browser_handle(&self) -> BrowserWindowHandle { self.handle.clone() }
}

impl BrowserWindowHandle {
	/// Returns the application handle associated with this browser window.
	pub fn app(&self) -> ApplicationHandle { ApplicationHandle::new(self.inner.window().app()) }

	pub fn close(self) { self.inner.window().destroy() }

	/// Executes the given javascript code and returns the output as a string.
	/// If you don't need the result, see `exec_js`.
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
		H: FnOnce(BrowserWindowHandle, Result<JsValue, JsEvaluationError>) + 'a,
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

	pub fn window(&self) -> WindowHandle { WindowHandle::new(self.inner.window()) }
}

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
		F: FnOnce(BrowserWindowHandle) -> R + Send + 'a,
		R: Send,
	{
		BrowserDelegateFuture::new(self.handle.clone(), func)
	}

	/// Executes the given async closure `func` on the GUI thread, and gives
	/// back the result when done.
	/// Also see `ApplicationThreaded::delegate_async`.
	pub fn delegate_async<'a, C, F, R>(&self, func: C) -> DelegateFutureFuture<'a, R>
	where
		C: FnOnce(BrowserWindowHandle) -> F + Send + 'a,
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
		F: FnOnce(BrowserWindowHandle) + Send + 'a,
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
		C: FnOnce(BrowserWindowHandle) -> F + Send + 'a,
		F: Future<Output = ()> + 'static,
	{
		let handle = UnsafeSend::new(self.handle.clone());

		self.app().dispatch(move |a| {
			a.spawn(func(handle.i));
		})
	}

	fn new(handle: BrowserWindowHandle) -> Self { Self { handle } }
}

#[cfg(feature = "threadsafe")]
impl HasAppHandle for BrowserWindowThreaded {
	fn app_handle(&self) -> ApplicationHandle { self.handle.app_handle() }
}

#[cfg(feature = "threadsafe")]
impl OwnedWindow for BrowserWindowThreaded {
	fn window_handle(&self) -> WindowHandle { self.handle.window() }
}

#[cfg(feature = "threadsafe")]
impl OwnedBrowserWindow for BrowserWindowThreaded {
	fn browser_handle(&self) -> BrowserWindowHandle { self.handle.clone() }
}

impl BrowserWindowHandle {
	fn new(inner_handle: BrowserWindowImpl) -> Self {
		Self {
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
	fn app_handle(&self) -> ApplicationHandle { ApplicationHandle::new(self.inner.window().app()) }
}

fn eval_js_callback<H>(
	_handle: BrowserWindowImpl, cb_data: *mut (), result: Result<JsValue, JsEvaluationError>,
) where
	H: FnOnce(BrowserWindowHandle, Result<JsValue, JsEvaluationError>),
{
	let data_ptr = cb_data as *mut H;
	let data = unsafe { Box::from_raw(data_ptr) };

	let handle = BrowserWindowHandle::new(_handle);

	(*data)(handle, result);
}
