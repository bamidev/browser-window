//! Browser Window is a Rust crate that allows you to have and manipulate windows with browsers in them.
//! Just like Electron, you can build graphical user interfaces with HTML/CSS/JS technology, but you can also use it to just have some browser functionality in your application.
//!
//! To start using Browser Window, you need to start it (before anything else) and run it, and preferably on the main thread.
//!
//! Your program might look something like this:
//! ```
//! use browser_window::*;
//! use std::process::exit;
//!
//! fn main() {
//! 	let runtime = Runtime::start();
//!
//! 	let exit_code = runtime.run(|app| {
//!
//! 		BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com/".into()) )
//! 			.spawn( &app, |browser| {
//! 				browser.exec_js("document.getElementById('search_form_input_homepage').value = 'Hello World!'");
//! 			});
//! 	});
//!
//! 	exit( exit_code );
//! }
//! ```
//!
//! For an example that uses Browser Window in an asynchronous context, see [this example code](https://github.com/bamilab/browser-window/blob/master/example/src/main.rs).

mod application;
mod browser;
mod common;
//mod treelink;



pub use application::{
	Application,
	ApplicationAsync,
	ApplicationDispatchFuture,
	Runtime
};
pub use browser::{
	Browser,
	BrowserAsync,
	BrowserDispatchFuture
};
pub use browser::builder::{
	BrowserBuilder,
	Source
};
