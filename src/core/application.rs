#[cfg(not(feature = "gtk"))]
pub mod c;
#[cfg(feature = "gtk")]
pub mod gtk;

use std::{
	os::raw::{c_char, c_int},
	time::Duration,
};

#[cfg(not(feature = "gtk"))]
pub use c::ApplicationImpl;
#[cfg(feature = "gtk")]
pub use gtk::ApplicationImpl;

use crate::{application::ApplicationSettings, error::Result};


pub trait ApplicationExt: Clone {
	/// Asserts if not on the GUI thread
	fn assert_correct_thread(&self);
	/// Dispatches work to be executed on the GUI thread.
	fn dispatch(&self, work: fn(ApplicationImpl, *mut ()), data: *mut ()) -> bool;
	/// Dispatches work to be executed on the GUI thread, but delayed by the
	/// specified number of milliseconds.
	fn dispatch_delayed(
		&self, work: fn(ApplicationImpl, *mut ()), data: *mut (), delay: Duration,
	) -> bool;
	/// Causes the main loop to exit and lets it return the given code.
	fn exit(&self, exit_code: i32);
	/// Same as `exit`, but is thread-safe.
	fn exit_threadsafe(self: &Self, exit_code: i32);
	/// Shuts down all application processes and performs necessary clean-up
	/// code.
	fn free(&self) {}
	fn initialize(
		argc: c_int, argv: *mut *mut c_char, settings: &ApplicationSettings,
	) -> Result<ApplicationImpl>;
	/// When this is called, the runtime will exit as soon as there are no more
	/// windows left.
	fn mark_as_done(&self);
	/// Runs the main loop.
	/// This blocks until the application is exitting.
	fn run(&self, on_ready: fn(ApplicationImpl, *mut ()), data: *mut ()) -> i32;
}
