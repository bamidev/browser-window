pub mod c;


use std::{
	borrow::Cow,
	time::SystemTime
};

pub use c::*;



pub type CookieStorageCallbackFn = unsafe fn( cj: CookieJarImpl, data: *mut (), Result<(), CookieStorageError> );

pub trait CookieExt {
	fn new(name: &str, value: &str) -> CookieImpl;

	fn creation_time(&self) -> SystemTime;
	fn expires(&self) -> Option<SystemTime>;
	fn domain<'a>(&'a self) -> Cow<'a, str>;
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
	fn global() -> CookieJarImpl;
	fn iterator<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIteratorImpl<'a>;
	fn store(&self, url: &str, cookie: &CookieImpl, success_cb: Option<CookieStorageCallbackFn>, cb_data: *mut ());
}

pub trait CookieIteratorExt {
	fn next(&mut self) -> Option<CookieImpl>;
}

#[derive(Debug)]
pub enum CookieStorageError {
	Unknown
}