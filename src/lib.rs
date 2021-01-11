//! Browser Window is a Rust crate that allows you to utilize a browser engine to build graphical interfaces.
//! The idea is similar to Electron.
//!
//! *Warning*: At the moment only Windows is suppored, but GTK support is on the way.
//!
//! Moreover, Browser Window uses Blink as the browser engine.
//! The Chromium Embedding Framework (CEF) is used as a dependency.
//! To get CEF set up properly, take a look [here](https://github.com/bamilab/browser-window/tree/master/docs/getting-started).
//!
//! To start building apps with Browser Window, take a quick look at the [`application`](application/index.html) module.
//!
//! For an rich example, see [this example code](https://github.com/bamilab/browser-window/blob/master/example/).


#[macro_use]
mod prop;

pub mod application;
pub mod browser;
pub mod event;
pub mod prelude;
pub mod window;

#[cfg(test)]
mod tests;



#[cfg(feature = "threadsafe")]
mod delegate;
#[cfg(feature = "threadsafe")]
pub use delegate::{
	DelegateError,
	DelegateFuture,
	DelegateFutureFuture
};
pub use prop::Property;
