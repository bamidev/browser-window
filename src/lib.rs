//! Browser Window is a Rust crate that allows you to have and manipulate windows with browsers in them.
//! Just like Electron, you can build graphical user interfaces with HTML/CSS/JS technology, but you can also use it to just have some browser functionality in your application.
//! 
//! To start using Browser Window, you need to start it before anything else, and preferably on the main thread.
//! To do this we use `Application::start()`, which gives you an application handle that can be used to create the browser windows.
//!
//! Your program might look something like this:
//! ```
//! use browser_window::*;
//!
//! fn main() {
//! 	let app = Application::start();
//!
//! 	BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com/".to_owned()) )
//! 	.spawn( &app, |browser| {
//! 		browser.exec_js(" ... ");
//! 	});
//! }
//! ```
//! 
//! For an example that uses Browser Window in an asynchronous context, see [this example code](https://github.com/bamilab/browser-window/blob/master/example/src/main.rs).

#![cfg_attr(feature = "nightly", feature(negative_impls))]

mod application;
mod browser_window;
mod common;



pub use application::{
	Application,
	ApplicationAsync,
	ApplicationDispatchFuture,
	ApplicationHandle
};
pub use browser_window::{
	BrowserWindow,
	BrowserWindowAsync,
	BrowserWindowDispatchFuture,
	BrowserWindowHandle
};
pub use browser_window::builder::{
	BrowserWindowBuilder,
	Source
};
