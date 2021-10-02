//! Browser Window is a Rust crate that allows you to utilize a browser engine to build graphical interfaces.
//! The idea is similar to Electron.
//!
//! # Warning
//! _Browser Window_ is still in alpha stage of development.
//! The API is bound to have breaking changes every minor version increment.
//! Once version 1.0.0 is released, minor version increments will be backwards-compatible.
//!
//! Moreover, the only supported browser engine right now is Chromium's Blink.
//! Therefore, the Chromium Embedding Framework (CEF) is used as a dependency.
//! To get CEF set up properly, take a look [here](https://github.com/bamilab/browser-window/tree/master/docs/GETTING-STARTED.md).
//! 
//! # Warning
//! Due to insufficient support of GTK for CEF, some windowing features are only available on Windows.
//! 
//! # Getting Started
//! To start building apps with Browser Window, take a quick look at the [`application`](application/index.html) module.
//! 
//! For a rich example, see [this example code](https://github.com/bamilab/browser-window/tree/master/examples).


#[macro_use]
mod prop;

pub mod application;
pub mod browser;
pub mod event;
pub mod prelude;
pub mod window;



#[cfg(feature = "threadsafe")]
mod delegate;
#[cfg(feature = "threadsafe")]
pub use delegate::{
	DelegateError,
	DelegateFuture,
	DelegateFutureFuture
};
pub use prop::Property;
