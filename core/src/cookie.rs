pub mod c;


use std::borrow::{
	Cow
};

pub use c::{CookieImpl, CookieJarImpl, CookieIteratorImpl};



pub trait CookieExt {
	fn name<'a>(&'a self) -> Cow<'a, str>;
	fn value<'a>(&'a self) -> Cow<'a, str>;
}

pub trait CookieJarExt {
	fn global() -> CookieJarImpl;
	fn iterator<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIteratorImpl<'a>;
}

pub trait CookieIteratorExt {
	fn next(&mut self) -> Option<CookieImpl>;
}