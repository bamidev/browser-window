use browser_window_core::cookie::*;

use std::borrow::Cow;



pub struct Cookie {
	inner: CookieImpl
}

pub struct CookieJar {
	inner: CookieJarImpl
}

pub struct CookieIterator<'a> {
	inner: CookieIteratorImpl<'a>
}



impl Cookie {
	pub fn name<'a>(&'a self) -> Cow<'a, str> {
		return self.inner.name()
	}

	pub fn value<'a>(&'a self) -> Cow<'a, str> {
		return self.inner.value()
	}
}

impl<'a> Iterator for CookieIterator<'a> {
	type Item = Cookie;

	fn next(&mut self) -> Option<Cookie> {
		self.inner.next().map(|inner| Cookie { inner })
	}
}

impl CookieJar {
	pub fn global() -> Self {
		Self {
			inner: CookieJarImpl::global()
		}
	}

	pub fn iter<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIterator<'a> {
		let inner = self.inner.iterator(url, include_http_only);

		CookieIterator {
			inner
		}
	}
}