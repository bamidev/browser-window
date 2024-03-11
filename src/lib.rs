//! _BrowserWindow_ is a Rust crate that allows you to utilize a browser engine
//! to build graphical interfaces, similar to Electron.js.
//! You create your user interface with HTML, CSS and JavaScript.
//! Then, you can communicate information to JavaScript and back to Rust.
//!
//! Pick the underlying browser framework by setting feature `cef`, `webkitgtk`
//! or `edge2`. For more info on which on you should choose and how to set them
//! up, check [this guide](https://github.com/bamidev/browser-window/tree/master/docs/GETTING-STARTED.md).

//! # Getting Started
//! To start building apps with Browser Window, take a quick look at the
//! [`application`](application/index.html) module.
//! Otherwise, here is a quick example:
//! ```
//! use browser_window::{application::*, browser::*, prelude::*};
//!
//! fn main() {
//! 	let app = Application::initialize(&ApplicationSettings::default()).unwrap();
//! 	let runtime = app.start();
//!
//! 	runtime.run(|app| {
//! 		let mut bwb = BrowserWindowBuilder::new(Source::File("file:///my-file.html".into()));
//! 		bwb.dev_tools(true);
//! 		bwb.size(800, 600);
//! 		bwb.title("Example");
//! 		bwb.build_sync(&app, |bw| {
//! 			bw.on_message().register(|h, e| {
//! 				match e.cmd.as_str() {
//! 					"command_one" => {
//! 						h.eval_js(&format!(
//! 							"js_function({}, {}, {})",
//! 							1,
//! 							"'two'",
//! 							JsValue::String("𝟛\n".into()) // Gets properly formatted to a JS string literal
//! 						));
//! 					}
//! 					"command_two" => {
//! 						// ... do something else here ...
//! 					}
//! 					_ => {}
//! 				}
//! 			});
//!
//! 			bw.show();
//! 		});
//! 	});
//! 	app.finish();
//! }
//! ```
//!
//! For more detailed example code, see [this example code](https://github.com/bamidev/browser-window/tree/master/examples).
//!
//! Or, for a very simple example of a browser frame, [look at this](https://github.com/bamidev/stonenet/blob/dev/desktop/src/main.rs).
//!
//! # Thread safety
//! To use the threadsafe version of _BrowserWindow_, enable feature
//! `threadsafe`. This will use [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
//! instead of [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)
//! internally, and will enable the
//! [`BrowserWindowThreaded`](browser/struct.BrowserWindow.html) and
//! [`ApplicationHandleThreaded`](browser/struct.BrowserWindowThreaded.html)
//! handles. It will also require closures to be `Send`. Docs.rs will show the
//! threadsafe versions of everything. If you need to know how everything is
//! compiled in the non-threadsafe version, you need to invoke `cargo doc
//! --open` in the git repo yourself.
//!
//! # Events
//! To learn how to use events, take a quick look at the
//! [`event`](event/index.html) module.

mod core;
#[cfg(test)]
mod tests;

pub mod application;
pub mod browser;
pub mod cookie;
pub mod error;
pub mod event;
pub mod javascript;
pub mod prelude;
pub(crate) mod rc;
pub mod window;

#[cfg(feature = "threadsafe")]
mod delegate;
#[cfg(feature = "threadsafe")]
pub use delegate::{DelegateError, DelegateFuture, DelegateFutureFuture};

mod common;
pub use common::*;
