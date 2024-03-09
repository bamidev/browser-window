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
use crate::{browser::*, prelude::JsValue, rc::*};


//pub type BrowserWindowEventHandler<'a, A> = EventHandler<'a,
// BrowserWindowHandle, A>;

pub type BrowserWindowOptions = cbw_BrowserWindowOptions;

/// The data that is passed to the C FFI handler function
pub(crate) struct BrowserUserData {
	pub(crate) _handle: Rc<BrowserWindowOwner>,
}

pub type CreationCallbackFn = fn(bw: BrowserWindowImpl, data: *mut ());
pub type EvalJsCallbackFn =
	fn(bw: BrowserWindowImpl, data: *mut (), result: Result<JsValue, JsEvaluationError>);

pub trait BrowserWindowEventExt {
	fn on_address_changed(&self, _handle: Weak<BrowserWindowOwner>) -> AddressChangedEvent {
		unimplemented!();
	}
	fn on_console_message(&self, _handle: Weak<BrowserWindowOwner>) -> ConsoleMessageEvent {
		unimplemented!();
	}
	fn on_favicon_changed(&self, _handle: Weak<BrowserWindowOwner>) -> FaviconChangedEvent {
		unimplemented!();
	}
	fn on_fullscreen_mode_changed(
		&self, _handle: Weak<BrowserWindowOwner>,
	) -> FullscreenModeChangedEvent {
		unimplemented!();
	}
	fn on_loading_progress_changed(
		&self, _handle: Weak<BrowserWindowOwner>,
	) -> LoadingProgressChangedEvent {
		unimplemented!();
	}
	fn on_message(&self, _handle: Weak<BrowserWindowOwner>) -> MessageEvent;
	fn on_navigation_end(&self, _handle: Weak<BrowserWindowOwner>) -> NavigationEndEvent {
		unimplemented!();
	}
	fn on_navigation_start(&self, _handle: Weak<BrowserWindowOwner>) -> NavigationStartEvent {
		unimplemented!();
	}
	fn on_page_title_changed(&self, _handle: Weak<BrowserWindowOwner>) -> PageTitleChangedEvent {
		unimplemented!();
	}
	fn on_status_message(&self, _handle: Weak<BrowserWindowOwner>) -> StatusMessageEvent {
		unimplemented!();
	}
	fn on_tooltip(&self, _handle: Weak<BrowserWindowOwner>) -> TooltipEvent {
		unimplemented!();
	}
}

pub trait BrowserWindowExt: BrowserWindowEventExt + Clone {
	fn cookie_jar(&self) -> Option<CookieJarImpl>;

	/// Executes the given JavaScript string.
	/// The result will be provided by invoking the callback function.
	fn eval_js(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ());

	/// Like `eval_js`, except it can be called from any thread.
	fn eval_js_threadsafe(&self, js: &str, callback: EvalJsCallbackFn, callback_data: *mut ());

	fn free(&self);

	/// Causes the browser to navigate to the given URI.
	fn navigate(&self, uri: &str);

	fn url<'a>(&'a self) -> Cow<'a, str>;

	/// Gives a handle to the underlying window.
	fn window(&self) -> WindowImpl;

	fn new(
		app: ApplicationImpl, parent: WindowImpl, source: Source, title: &str, width: Option<u32>,
		height: Option<u32>, options: &WindowOptions,
		browser_window_options: &BrowserWindowOptions, creation_callback: CreationCallbackFn, callback_data: *mut (),
	);
}


impl BrowserWindowImpl {
	pub(crate) fn free_user_data(user_data: *mut ()) {
		let ptr = user_data as *mut BrowserUserData;
		unsafe {
			let _ = Box::from_raw(ptr);
		}
	}
}
