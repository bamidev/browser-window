use std::{
	ffi::{c_char, c_int},
	sync::{
		Arc,
		atomic::{AtomicI32, Ordering},
	},
	time::Duration,
};

use gtk::prelude::{ApplicationExt, ApplicationExtManual};

use super::{super::error::*, ApplicationSettings};

#[derive(Clone)]
pub struct ApplicationImpl {
	pub inner: gtk::Application,
	pub exit_code: Arc<AtomicI32>,
}

impl super::ApplicationExt for ApplicationImpl {
	fn assert_correct_thread(&self) { unimplemented!() }

	fn dispatch(&self, work: fn(ApplicationImpl, *mut ()), data: *mut ()) -> bool {
		let this = self.clone();
		gtk::glib::source::idle_add_local_once(move || work(this, data));
		true
	}

	fn dispatch_delayed(
		&self, work: fn(ApplicationImpl, *mut ()), data: *mut (), delay: Duration,
	) -> bool {
		let this = self.clone();
		gtk::glib::source::timeout_add_local_once(delay, move || work(this, data));
		true
	}

	fn exit(&self, exit_code: i32) {
		self.exit_code.store(exit_code, Ordering::Relaxed);
		self.inner.quit();
	}

	fn exit_threadsafe(&self, exit_code: i32) { self.exit(exit_code); }

	fn free(&self) {}

	fn initialize(
		_argc: c_int, _argv: *mut *mut c_char, _settings: &ApplicationSettings,
	) -> Result<Self> {
		let inner = gtk::Application::builder().build();
		Ok(Self {
			inner,
			exit_code: Arc::new(AtomicI32::new(0)),
		})
	}

	fn mark_as_done(&self) {}

	fn run(&self, on_ready: fn(ApplicationImpl, *mut ()), data: *mut ()) -> i32 {
		let this = self.clone();
		self.inner.connect_activate(move |_| {
			on_ready(this.clone(), data);
		});
		self.inner.run();
		self.exit_code.load(Ordering::Relaxed)
	}
}
