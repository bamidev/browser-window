use std::{
	borrow::Cow,
	ffi::c_void,
	mem::MaybeUninit,
	ops::Add,
	os::raw::c_uint,
	ptr,
	time::{Duration, SystemTime},
};

use browser_window_c::*;

use super::*;

pub struct CookieImpl(pub(crate) *mut cbw_Cookie);

pub struct CookieMutImpl(CookieImpl);

pub struct CookieJarImpl(pub(crate) *mut cbw_CookieJar);

pub struct CookieIteratorImpl(pub(crate) *mut cbw_CookieIterator);

struct CookieStorageCallbackData {
	callback: CookieStorageCallbackFn,
	data: *mut (),
}

struct CookieDeleteCallbackData {
	callback: CookieDeleteCallbackFn,
	data: *mut (),
}

struct CookieIteratorNextCallbackData {
	callback: CookieIteratorNextCallbackFn,
	data: *mut (),
}

impl CookieExt for CookieImpl {
	fn creation_time(&self) -> SystemTime {
		let timestamp = unsafe { cbw_Cookie_getCreationTime(self.0) };

		SystemTime::UNIX_EPOCH.add(Duration::from_millis(timestamp))
	}

	fn expires(&self) -> Option<SystemTime> {
		let timestamp = unsafe { cbw_Cookie_getExpires(self.0) };

		if timestamp != 0 {
			Some(SystemTime::UNIX_EPOCH.add(Duration::from_millis(timestamp)))
		} else {
			None
		}
	}

	fn domain<'a>(&'a self) -> Cow<'a, str> {
		let slice;
		let owned;
		unsafe {
			let mut slice_uninit: MaybeUninit<cbw_StrSlice> = MaybeUninit::uninit();
			owned = cbw_Cookie_getDomain(self.0, slice_uninit.as_mut_ptr());
			slice = slice_uninit.assume_init();
		}

		if owned > 0 {
			let string: String = slice.into();
			unsafe { cbw_string_free(slice) };
			string.into()
		} else {
			let string: &str = slice.into();
			string.into()
		}
	}

	fn free(&mut self) { unsafe { cbw_Cookie_free(self.0) }; }

	fn is_http_only(&self) -> bool { (unsafe { cbw_Cookie_isHttpOnly(self.0) }) > 0 }

	fn is_secure(&self) -> bool { (unsafe { cbw_Cookie_isSecure(self.0) }) > 0 }

	fn name<'a>(&'a self) -> Cow<'a, str> {
		let slice;
		let owned;
		unsafe {
			let mut slice_uninit: MaybeUninit<cbw_StrSlice> = MaybeUninit::uninit();
			owned = cbw_Cookie_getName(self.0, slice_uninit.as_mut_ptr());
			slice = slice_uninit.assume_init();
		}

		if owned > 0 {
			let string: String = slice.into();
			unsafe { cbw_string_free(slice) };
			string.into()
		} else {
			let string: &str = slice.into();
			string.into()
		}
	}

	fn new(name: &str, value: &str) -> Self {
		let inner = unsafe { cbw_Cookie_new(name.into(), value.into()) };

		Self(inner)
	}

	fn path<'a>(&'a self) -> Cow<'a, str> {
		let slice;
		let owned;
		unsafe {
			let mut slice_uninit: MaybeUninit<cbw_StrSlice> = MaybeUninit::uninit();
			owned = cbw_Cookie_getPath(self.0, slice_uninit.as_mut_ptr());
			slice = slice_uninit.assume_init();
		}

		if owned > 0 {
			let string: String = slice.into();
			unsafe { cbw_string_free(slice) };
			string.into()
		} else {
			let string: &str = slice.into();
			string.into()
		}
	}

	fn value<'a>(&'a self) -> Cow<'a, str> {
		let slice;
		let owned;
		unsafe {
			let mut slice_uninit: MaybeUninit<cbw_StrSlice> = MaybeUninit::uninit();
			owned = cbw_Cookie_getValue(self.0, slice_uninit.as_mut_ptr());
			slice = slice_uninit.assume_init();
		}

		if owned > 0 {
			let string: String = slice.into();
			unsafe { cbw_string_free(slice) };
			string.into()
		} else {
			let string: &str = slice.into();
			string.into()
		}
	}

	fn make_http_only(&mut self) { unsafe { cbw_Cookie_makeHttpOnly(self.0) }; }

	fn make_secure(&mut self) { unsafe { cbw_Cookie_makeSecure(self.0) }; }

	fn set_creation_time(&mut self, time: &SystemTime) {
		unsafe {
			cbw_Cookie_setCreationTime(
				self.0,
				time.duration_since(SystemTime::UNIX_EPOCH)
					.unwrap()
					.as_millis() as _,
			)
		};
	}

	fn set_expires(&mut self, time: &SystemTime) {
		unsafe {
			cbw_Cookie_setExpires(
				self.0,
				time.duration_since(SystemTime::UNIX_EPOCH)
					.unwrap()
					.as_millis() as _,
			)
		};
	}

	fn set_domain(&mut self, domain: &str) {
		unsafe { cbw_Cookie_setDomain(self.0, domain.into()) };
	}

	fn set_name(&mut self, name: &str) { unsafe { cbw_Cookie_setName(self.0, name.into()) }; }

	fn set_path(&mut self, path: &str) { unsafe { cbw_Cookie_setPath(self.0, path.into()) }; }

	fn set_value(&mut self, value: &str) { unsafe { cbw_Cookie_setValue(self.0, value.into()) }; }
}

impl CookieJarExt for CookieJarImpl {
	fn delete(
		&mut self, url: &str, name: &str, complete_cb: CookieDeleteCallbackFn, cb_data: *mut (),
	) {
		let data = Box::into_raw(Box::new(CookieDeleteCallbackData {
			callback: complete_cb,
			data: cb_data,
		}));

		unsafe {
			cbw_CookieJar_delete(
				self.0,
				url.into(),
				name.into(),
				Some(ffi_cookie_delete_callback_handler),
				data as _,
			)
		};
	}

	fn free(&mut self) { unsafe { cbw_CookieJar_free(self.0) }; }

	fn global() -> Option<CookieJarImpl> {
		let inner = unsafe { cbw_CookieJar_newGlobal() };

		if inner != ptr::null_mut() {
			Some(CookieJarImpl(inner))
		} else {
			None
		}
	}

	fn iterator<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIteratorImpl {
		unsafe {
			let mut iterator: *mut cbw_CookieIterator = ptr::null_mut();
			cbw_CookieJar_iterator(
				self.0,
				&mut iterator as _,
				if include_http_only { 1 } else { 0 },
				url.into(),
			);

			return CookieIteratorImpl(iterator);
		}
	}

	fn iterator_all<'a>(&'a self) -> CookieIteratorImpl {
		let mut iterator = CookieIteratorImpl(ptr::null_mut());
		unsafe {
			cbw_CookieJar_iteratorAll(self.0, &mut iterator.0);
		}
		iterator
	}

	fn store(
		&mut self, url: &str, cookie: &CookieImpl, complete_cb: Option<CookieStorageCallbackFn>,
		cb_data: *mut (),
	) {
		let data = if !complete_cb.is_none() {
			Box::into_raw(Box::new(CookieStorageCallbackData {
				callback: complete_cb.unwrap(),
				data: cb_data,
			}))
		} else {
			ptr::null_mut()
		};

		unsafe {
			let mut error = cbw_CookieJar_store(
				self.0,
				url.into(),
				cookie.0,
				complete_cb.map(|_| ffi_cookie_storage_callback_handler as _),
				data as _,
			);

			if error.code != 0 && !complete_cb.is_none() {
				(complete_cb.unwrap())(
					CookieJarImpl(self.0),
					cb_data,
					Err(CookieStorageError::Unknown),
				);
			}

			cbw_Err_free(&mut error);
		}
	}
}

impl CookieIteratorExt for CookieIteratorImpl {
	fn free(&mut self) { unsafe { cbw_CookieIterator_free(self.0) } }

	fn next(&mut self, on_next: CookieIteratorNextCallbackFn, cb_data: *mut ()) -> bool {
		let data = Box::into_raw(Box::new(CookieIteratorNextCallbackData {
			callback: on_next,
			data: cb_data,
		}));

		let success = unsafe {
			cbw_CookieIterator_next(self.0, Some(ffi_cookie_iterator_next_handler), data as _)
		};

		return success > 0;
	}
}

unsafe extern "C" fn ffi_cookie_storage_callback_handler(
	cookie_jar: *mut cbw_CookieJar, _data: *mut c_void, error: cbw_Err,
) {
	let data_ptr = _data as *mut CookieStorageCallbackData;
	let data: Box<CookieStorageCallbackData> = Box::from_raw(data_ptr);

	let handle = CookieJarImpl(cookie_jar);
	let result = if error.code == 0 {
		Ok(())
	} else {
		Err(CookieStorageError::Unknown)
	};

	(data.callback)(handle, data.data, result);
}

unsafe extern "C" fn ffi_cookie_delete_callback_handler(
	cookie_jar: *mut cbw_CookieJar, _data: *mut c_void, deleted: c_uint,
) {
	let data_ptr = _data as *mut CookieDeleteCallbackData;
	let data: Box<CookieDeleteCallbackData> = Box::from_raw(data_ptr);

	let handle = CookieJarImpl(cookie_jar);

	(data.callback)(handle, data.data, deleted as _);
}

unsafe extern "C" fn ffi_cookie_iterator_next_handler(
	cookie_iterator: *mut cbw_CookieIterator, _data: *mut c_void, inner: *mut cbw_Cookie,
) {
	let data_ptr = _data as *mut CookieIteratorNextCallbackData;
	let data: Box<CookieIteratorNextCallbackData> = Box::from_raw(data_ptr);

	let handle = CookieIteratorImpl(cookie_iterator);
	let cookie = if inner == ptr::null_mut() {
		None
	} else {
		Some(CookieImpl(inner))
	};

	(data.callback)(handle, data.data, cookie);
}
