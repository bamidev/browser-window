mod c;


use std::{
	borrow::Cow,
	error::Error,
	fmt,
	time::SystemTime
};

pub use c::*;



pub type CookieStorageCallbackFn = unsafe fn( cj: CookieJarImpl, data: *mut (), Result<(), CookieStorageError> );
pub type CookieDeleteCallbackFn = unsafe fn(cj: CookieJarImpl, data: *mut (), deleted: usize);
pub type CookieIteratorNextCallbackFn = unsafe fn(cj: CookieIteratorImpl, data: *mut (), Option<CookieImpl>);

pub trait CookieExt {
	fn new(name: &str, value: &str) -> CookieImpl;

	fn creation_time(&self) -> SystemTime;
	fn expires(&self) -> Option<SystemTime>;
	fn domain<'a>(&'a self) -> Cow<'a, str>;
	fn free(&mut self);
	fn is_http_only(&self) -> bool;
	fn name<'a>(&'a self) -> Cow<'a, str>;
	fn path<'a>(&'a self) -> Cow<'a, str>;
	fn is_secure(&self) -> bool;
	fn value<'a>(&'a self) -> Cow<'a, str>;
	
	fn make_http_only(&mut self) -> &mut Self;
	fn make_secure(&mut self) -> &mut Self;
	fn set_creation_time(&mut self, time: &SystemTime) -> &mut Self;
	fn set_expires(&mut self, time: &SystemTime) -> &mut Self;
	fn set_domain(&mut self, domain: &str) -> &mut Self;
	fn set_name(&mut self, name: &str) -> &mut Self;
	fn set_path(&mut self, path: &str) -> &mut Self;
	fn set_value(&mut self, value: &str) -> &mut Self;
}

pub trait CookieJarExt {
	fn delete(&mut self, url: &str, name: &str, complete_cb: CookieDeleteCallbackFn, cb_data: *mut ());
	fn free(&mut self);
	fn global() -> CookieJarImpl;
	fn iterator<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIteratorImpl;
	fn iterator_all<'a>(&'a self) -> CookieIteratorImpl;
	fn store(&mut self, url: &str, cookie: &CookieImpl, success_cb: Option<CookieStorageCallbackFn>, cb_data: *mut ());
}

pub trait CookieIteratorExt {
	fn free(&mut self);
	fn next(&mut self, on_next: CookieIteratorNextCallbackFn, cb_data: *mut ()) -> bool;
}

#[derive(Debug)]
pub enum CookieStorageError {
	Unknown
}



impl fmt::Display for CookieStorageError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "unable to set cookie (url invalid?)")
	}
}

impl Error for CookieStorageError {
	fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}