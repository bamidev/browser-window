//! Browser Window is a Rust crate that allows you manipulate simple browsers contained within simple windows.
//! Just like Electron, you can use it to build graphical user interfaces with HTML/CSS/JS technology.
//!
//! *Warning*: At the moment only Windows is suppored. (GTK support is on the way!)
//!
//! Moreover, Browser Window uses the Chromium browser engine.
//! So Browser Window uses CEF as a dependency.
//! To get CEF set up properly, take a look [here](https://github.com/bamilab/browser-window/tree/master/docs/getting-started).
//!
//! To start building apps with Browser Window, take a quick look at the [`application`](application/index.html) module.
//!
//! For an example, see [this example code](https://github.com/bamilab/browser-window/blob/master/example/).

pub mod application;
pub mod browser;

#[cfg(test)]
mod tests;



mod common;
pub use common::{
	DelegateError,
	DelegateFuture,
	DelegateFutureFuture
};
