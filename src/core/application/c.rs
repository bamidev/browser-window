//! This module implements the `Application` trait with the corresponding
//! function definitions found in the C code base of `browser-window-c`.
//! All functions are basically wrapping the FFI provided by crate
//! `browser-window-c`.

use std::{
	os::raw::{c_char, c_int, c_void},
	path::PathBuf,
	ptr,
	time::Duration,
};

use super::{ApplicationExt, ApplicationSettings};
use crate::{error::*, prelude::*};

#[derive(Clone, Copy)]
pub struct ApplicationImpl {
	pub(crate) inner: *mut cbw_Application,
}

impl ApplicationExt for ApplicationImpl {
	fn assert_correct_thread(&self) { unsafe { cbw_Application_assertCorrectThread(self.inner) } }

	fn dispatch(&self, work: fn(ApplicationImpl, *mut ()), _data: *mut ()) -> bool {
		let data = Box::new(DispatchData {
			func: work,
			data: _data,
		});

		let data_ptr = Box::into_raw(data);

		unsafe {
			cbw_Application_dispatch(self.inner, Some(invocation_handler), data_ptr as _) != 0
		}
	}

	fn dispatch_delayed(
		&self, work: fn(ApplicationImpl, *mut ()), _data: *mut (), delay: Duration,
	) -> bool {
		let data = Box::new(DispatchData {
			func: work,
			data: _data,
		});

		let data_ptr = Box::into_raw(data);

		unsafe {
			cbw_Application_dispatchDelayed(
				self.inner,
				Some(invocation_handler),
				data_ptr as _,
				delay.as_millis() as _,
			) != 0
		}
	}

	fn exit(&self, exit_code: i32) { unsafe { cbw_Application_exit(self.inner, exit_code as _) } }

	fn exit_threadsafe(self: &Self, exit_code: i32) {
		unsafe { cbw_Application_exitAsync(self.inner, exit_code) }
	}

	fn initialize(
		argc: c_int, argv: *mut *mut c_char, settings: &ApplicationSettings,
	) -> Result<Self> {
		let exec_path: &str = match settings.engine_seperate_executable_path.as_ref() {
			None => "",
			Some(path) => path.to_str().unwrap(),
		};

		let c_settings = cbw_ApplicationSettings {
			engine_seperate_executable_path: exec_path.into(),
			resource_dir: settings
				.resource_dir
				.as_ref()
				.unwrap_or(&PathBuf::new())
				.as_os_str()
				.to_string_lossy()
				.as_ref()
				.into(),
			remote_debugging_port: settings.remote_debugging_port.unwrap_or(0)
		};

		let mut c_handle: *mut cbw_Application = ptr::null_mut();
		let c_err = unsafe { cbw_Application_initialize(&mut c_handle, argc, argv, &c_settings) };
		if c_err.code != 0 {
			return Err(c_err.into());
		}

		Ok(Self { inner: c_handle })
	}

	fn mark_as_done(&self) { unsafe { cbw_Application_markAsDone(self.inner) }; }

	fn run(&self, on_ready: fn(ApplicationImpl, *mut ()), _data: *mut ()) -> i32 {
		let data = Box::new(DispatchData {
			func: on_ready,
			data: _data,
		});

		let data_ptr = Box::into_raw(data);

		// The dispatch handler does exactly the same thing
		unsafe { cbw_Application_run(self.inner, Some(invocation_handler), data_ptr as _) }
	}
}

struct DispatchData {
	func: unsafe fn(ApplicationImpl, *mut ()),
	data: *mut (),
}

unsafe extern "C" fn invocation_handler(_handle: *mut cbw_Application, _data: *mut c_void) {
	let data_ptr = _data as *mut DispatchData;
	let data = Box::from_raw(data_ptr);
	let handle = ApplicationImpl { inner: _handle };

	(data.func)(handle, data.data);
}
