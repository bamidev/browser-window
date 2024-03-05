#[cfg(not(any(feature = "gtk", feature = "edge2")))]
pub mod c;
#[cfg(feature = "edge2")]
mod edge2;
#[cfg(feature = "gtk")]
mod webkit;

use std::borrow::Cow;

use browser_window_c::*;
#[cfg(not(any(feature = "gtk", feature = "edge2")))]
pub use c::{BrowserWindowImpl, JsEvaluationError};
#[cfg(feature = "edge2")]
pub use edge2::{BrowserWindowImpl, JsEvaluationError};
#[cfg(feature = "gtk")]
pub use webkit::{BrowserWindowImpl, JsEvaluationError};

use super::{
	super::event::*,
	application::ApplicationImpl,
	cookie::CookieJarImpl,
	window::{WindowImpl, WindowOptions},
};
use crate::{browser::*, prelude::JsValue};


pub type BrowserWindowEventHandler<'a, A> = EventHandler<'a, BrowserWindowHandle, A>;
pub type BrowserWindowEventHandlerCallback<'a, A> = EventHandlerCallback<'a, BrowserWindowHandle, A>;

pub type BrowserWindowOptions = cbw_BrowserWindowOptions;

pub type CreationCallbackFn = fn(bw: BrowserWindowImpl, data: *mut ());
pub type EvalJsCallbackFn =
	fn(bw: BrowserWindowImpl, data: *mut (), result: Result<JsValue, JsEvaluationError>);
pub type ExternalInvocationHandlerFn = fn(bw: BrowserWindowImpl, cmd: &str, args: Vec<JsValue>);

pub trait BrowserWindowEventExt {
	fn on_page_title_changed<'a>(&self, handle: &'a BrowserWindowHandle) -> PageTitleChangedEvent<'a> { unimplemented!(); }
	fn on_navigation_end<'a>(&self, handle: &'a BrowserWindowHandle) -> NavigationEndEvent<'a> { unimplemented!(); }
	fn on_navigation_start<'a>(&self, handle: &'a BrowserWindowHandle) -> NavigationStartEvent<'a> { unimplemented!(); }
	fn on_tooltip<'a>(&self, handle: &'a BrowserWindowHandle) -> TooltipEvent<'a> { unimplemented!(); }
}

pub trait BrowserWindowExt: BrowserWindowEventExt {
	fn cookie_jar(&self) -> Option<CookieJarImpl>;

	/// Executes the given JavaScript string.
	/// The result will be provided by invoking the callback function.
	fn eval_js(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ());

	/// Like `eval_js`, except it can be called from any thread.
	fn eval_js_threadsafe(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ());

	fn free(&self);

	/// Causes the browser to navigate to the given URI.
	fn navigate(&self, uri: &str);

	fn user_data(&self) -> *mut ();

	fn url<'a>(&'a self) -> Cow<'a, str>;

	/// Gives a handle to the underlying window.
	fn window(&self) -> WindowImpl;
}
