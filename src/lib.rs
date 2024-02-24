//! _Browser Window_ is a Rust crate that allows you to utilize a browser engine
//! to build graphical interfaces. The idea is similar to Electron.js.
//! You create your user interface with HTML, CSS and JavaScript.
//! Then, you can communicate information to JavaScript and back to Rust.
//!
//! Pick the underlying browser framework by setting feature `webkitgtk` or
//! `cef`. For more info on how to them up, check [this guide](https://github.com/bamidev/browser-window/tree/master/docs/GETTING-STARTED.md).

//! # Getting Started
//! To start building apps with Browser Window, take a quick look at the
//! [`application`](application/index.html) module.
//!
//! For more detailed example code, see [this example code](https://github.com/bamidev/browser-window/tree/master/examples).
//!
//! # Thread safety
//! To use the threadsafe version of _Browser Window_, enable feature
//! `threadsafe`.

mod core;
#[macro_use]
mod prop;
#[cfg(test)]
mod tests;

pub mod application;
pub mod browser;
pub mod cookie;
pub mod error;
pub mod event;
pub mod javascript;
pub mod prelude;
pub mod window;

#[cfg(feature = "threadsafe")]
mod delegate;
#[cfg(feature = "threadsafe")]
pub use delegate::{DelegateError, DelegateFuture, DelegateFutureFuture};
pub use prop::Property;
