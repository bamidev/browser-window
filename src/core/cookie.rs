#[cfg(not(feature = "webkitgtk"))]
mod c;
#[cfg(feature = "webkitgtk")]
mod unsupported;

use std::{borrow::Cow, error::Error, fmt, time::SystemTime};

#[cfg(not(feature = "webkitgtk"))]
pub use c::*;
#[cfg(feature = "webkitgtk")]
pub use unsupported::*;

pub type CookieStorageCallbackFn =
	unsafe fn(cj: CookieJarImpl, data: *mut (), Result<(), CookieStorageError>);
pub type CookieDeleteCallbackFn = unsafe fn(cj: CookieJarImpl, data: *mut (), deleted: usize);
pub type CookieIteratorNextCallbackFn =
	unsafe fn(cj: CookieIteratorImpl, data: *mut (), Option<CookieImpl>);

pub trait CookieExt {
	fn new(_name: &str, _value: &str) -> CookieImpl {
		unimplemented!();
	}

	fn creation_time(&self) -> SystemTime {
		unimplemented!();
	}
	fn expires(&self) -> Option<SystemTime> {
		unimplemented!();
	}
	fn domain<'a>(&'a self) -> Cow<'a, str> {
		unimplemented!();
	}
	fn free(&mut self) {}
	fn is_http_only(&self) -> bool {
		unimplemented!();
	}
	fn name<'a>(&'a self) -> Cow<'a, str> {
		unimplemented!();
	}
	fn path<'a>(&'a self) -> Cow<'a, str> {
		unimplemented!();
	}
	fn is_secure(&self) -> bool {
		unimplemented!();
	}
	fn value<'a>(&'a self) -> Cow<'a, str> {
		unimplemented!();
	}

	fn make_http_only(&mut self) {
		unimplemented!();
	}
	fn make_secure(&mut self) {
		unimplemented!();
	}
	fn set_creation_time(&mut self, _time: &SystemTime) {
		unimplemented!();
	}
	fn set_expires(&mut self, _time: &SystemTime) {
		unimplemented!();
	}
	fn set_domain(&mut self, _domain: &str) {
		unimplemented!();
	}
	fn set_name(&mut self, _name: &str) {
		unimplemented!();
	}
	fn set_path(&mut self, _path: &str) {
		unimplemented!();
	}
	fn set_value(&mut self, _value: &str) {
		unimplemented!();
	}
}

pub trait CookieJarExt {
	fn delete(
		&mut self, _url: &str, _name: &str, _complete_cb: CookieDeleteCallbackFn, _cb_data: *mut (),
	) {
		unimplemented!();
	}
	fn free(&mut self) {}
	fn global() -> Option<CookieJarImpl> { None }
	fn iterator<'a>(&'a self, _url: &str, _include_http_only: bool) -> CookieIteratorImpl {
		unimplemented!();
	}
	fn iterator_all<'a>(&'a self) -> CookieIteratorImpl {
		unimplemented!();
	}
	fn store(
		&mut self, _url: &str, _cookie: &CookieImpl, _success_cb: Option<CookieStorageCallbackFn>,
		_cb_data: *mut (),
	) {
		unimplemented!();
	}
}

pub trait CookieIteratorExt {
	fn free(&mut self) {}
	fn next(&mut self, _on_next: CookieIteratorNextCallbackFn, _cb_data: *mut ()) -> bool {
		unimplemented!();
	}
}

#[derive(Debug)]
pub enum CookieStorageError {
	Unknown,
}

impl fmt::Display for CookieStorageError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "unable to set cookie (url invalid?)")
	}
}

impl Error for CookieStorageError {
	fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}
