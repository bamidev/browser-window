//! Module for dealing with cookies.

use browser_window_core::cookie::*;
use futures_channel::oneshot;

use std::{
	marker::PhantomData,
	ops::*,
	ptr
};

use super::application::ApplicationHandle;



pub struct Cookie {
	inner: CookieImpl
}

//pub struct CookieMut (Cookie);

pub struct CookieJar {
	inner: CookieJarImpl
}

pub struct CookieIterator<'a> {
	inner: CookieIteratorImpl,
	_phantom: PhantomData<&'a u8>
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

impl Drop for Cookie {
	fn drop(&mut self) {
		self.inner.free();
	}
}

impl<'a> CookieIterator<'a> {
	pub async fn next(&mut self) -> Option<Cookie> {
		let (tx, rx) = oneshot::channel::<Option<Cookie>>();

		let more = self._next(|result| {
			if let Err(_) = tx.send(result) {
				panic!("unable to send cookie iterator next result back");
			}
		});

		if !more {
			return None;
		}

		rx.await.unwrap()
	}

	fn _next<H>(&mut self, on_next: H) -> bool where
		H: FnOnce(Option<Cookie>)
	{
		let data = Box::into_raw(Box::new(
			on_next
		));

		let called_closure = self.inner.next(cookie_iterator_next_handler::<H>, data as _);

		if !called_closure {
			unsafe { Box::from_raw(data) };
		}

		called_closure
	}
}

impl<'a> Drop for CookieIterator<'a> {
	fn drop(&mut self) {
		self.inner.free();
	}
}

impl CookieJar {

	pub async fn find(&self, name: &str, url: &str, include_http_only: bool) -> Option<Cookie> {
		let mut iter = self.iter(url, include_http_only);

		while let Some(cookie) = iter.next().await {
			if cookie.name() == name {
				return Some(cookie)
			}
		}

		None
	}

	pub async fn find_from_all(&self, name: &str) -> Option<Cookie> {
		let mut iter = self.iter_all();

		while let Some(cookie) = iter.next().await {
			if cookie.name() == name {
				return Some(cookie)
			}
		}

		None
	}

	pub fn global(_app: &ApplicationHandle) -> Self {
		Self {
			inner: CookieJarImpl::global()
		}
	}

	pub fn iter<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIterator<'a> {
		let inner = self.inner.iterator(url, include_http_only);

		CookieIterator {
			inner,
			_phantom: PhantomData
		}
	}

	pub fn iter_all<'a>(&'a self) -> CookieIterator<'a> {
		let inner = self.inner.iterator_all();

		CookieIterator {
			inner,
			_phantom: PhantomData
		}
	}

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

impl Drop for CookieJar {
	fn drop(&mut self) {
		self.inner.free();
	}
}



unsafe fn cookie_store_callback<'a, H>( _handle: CookieJarImpl, cb_data: *mut (), result: Result<(), CookieStorageError> ) where
	H: FnOnce(Result<(), CookieStorageError>) + 'a
{
	let data_ptr = cb_data as *mut H;
	let data: Box<H> = Box::from_raw( data_ptr );

	(*data)( result );
}

unsafe fn cookie_iterator_next_handler<H>(_handle: CookieIteratorImpl, cb_data: *mut (), cookie: Option<CookieImpl>) where
	H: FnOnce(Option<Cookie>)
{
	let data_ptr = cb_data as *mut H;
	let data: Box<H> = Box::from_raw(data_ptr);

	(*data)(cookie.map(|c| Cookie {inner: c}));
}