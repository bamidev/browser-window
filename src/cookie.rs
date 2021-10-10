//! Module for dealing with cookies.

use browser_window_core::cookie::*;
use futures_channel::oneshot;

use std::{
	ops::*,
	ptr
};



pub struct Cookie {
	inner: CookieImpl
}

//pub struct CookieMut (Cookie);

pub struct CookieJar {
	inner: CookieJarImpl
}

pub struct CookieIterator<'a> {
	inner: CookieIteratorImpl<'a>
}

//pub struct CookieIteratorMut<'a> (CookieIterator<'a>);


impl Cookie {
	pub fn new(name: &str, value: &str) -> Self {
		Self { inner: CookieImpl::new(name, value) }
	}
}

impl Deref for Cookie {
	type Target = CookieImpl;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Cookie {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

/*impl CookieMut {
	pub fn set_value(&mut self, value: &str) {
		self.0.inner.set_value(value)
	}
}

impl Deref for CookieMut {
	type Target = Cookie;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}*/

impl<'a> Iterator for CookieIterator<'a> {
	type Item = Cookie;

	fn next(&mut self) -> Option<Cookie> {
		self.inner.next().map(|inner| Cookie { inner })
	}
}

/*impl<'a> Iterator for CookieIteratorMut<'a> {
	type Item = CookieMut;

	fn next(&mut self) -> Option<CookieMut> {
		self.0.next().map(|inner| CookieMut(inner))
	}
}*/

impl CookieJar {
	pub fn find(&self, name: &str, path: &str, include_http_only: bool) -> Option<Cookie> {
		for cookie in self.iter(path, include_http_only) {
			if cookie.name() == name {
				return Some(cookie)
			}
		}

		None
	}

	/*pub fn find_mut(&mut self, name: &str, path: &str, include_http_only: bool) -> Option<CookieMut> {
		for cookie in self.iter_mut(path, include_http_only) {
			if cookie.name() == name {
				return Some(cookie)
			}
		}

		None
	}*/

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

	/*pub fn iter_mut<'a>(&'a mut self, url: &str, include_http_only: bool) -> CookieIteratorMut<'a> {
		let inner = self.iter(url, include_http_only);

		CookieIteratorMut (inner)
	}*/

	fn _store<'a,H>(&mut self, url: &str, cookie: &Cookie, on_complete: H) where
		H: FnOnce(Result<(), CookieStorageError>) + 'a
	{
		let data = Box::into_raw(Box::new(on_complete));

		self.inner.store(url.into(), &cookie.inner, Some(cookie_store_callback::<'a,H>), data as _);
	}

	pub async fn store(&mut self, url: &str, cookie: &Cookie) -> Result<(), CookieStorageError> {
		let (tx, rx) = oneshot::channel::<Result<(), CookieStorageError>>();

		self._store(url, cookie, |result| {
			tx.send(result).expect("unable to retrieve cookie storage error");
		});

		rx.await.unwrap()
	}

	pub fn store_start(&mut self, url: &str, cookie: &Cookie) {
		self.inner.store(url.into(), &cookie.inner, None, ptr::null_mut());
	}
}



unsafe fn cookie_store_callback<'a, H>( _handle: CookieJarImpl, cb_data: *mut (), result: Result<(), CookieStorageError> ) where
	H: FnOnce(Result<(), CookieStorageError>) + 'a
{
	let data_ptr = cb_data as *mut H;
	let data: Box<H> = Box::from_raw( data_ptr );

	(*data)( result );
}