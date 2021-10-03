//! Browser Window is a Rust crate that allows you to utilize a browser engine to build graphical interfaces.
//! The idea is similar to Electron.js.
//! You create the interface with HTML, CSS and JavaScript.
//! Then, you can communicate information from JavaScript back to Rust, and from the other way around.
//!
//! The only supported browser engine right now is Chromium's Blink.
//! Therefore, the Chromium Embedding Framework (CEF) is used as a dependency.
//! To get CEF set up properly, take a look [here](https://github.com/bamilab/browser-window/tree/master/docs/GETTING-STARTED.md).
//! Moreover, because getting CEF to work properly is not always straightforward, [here](https://github.com/bamilab/browser-window/tree/master/docs/ISSUE-DIAGNOSIS.md) is a list of common issues you may run in to, and their solutions.
//! 
//! # Getting Started
//! To start building apps with Browser Window, take a quick look at the [`application`](application/index.html) module.
//! 
//! For a rich example, see [this example code](https://github.com/bamilab/browser-window/tree/master/examples).


#[macro_use]
mod prop;

pub mod application;
pub mod browser;
pub mod error;
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
