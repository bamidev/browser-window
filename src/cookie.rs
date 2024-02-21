//! Module for dealing with cookies.

use crate::core::cookie::*;
use futures_channel::oneshot;

use std::{
	marker::PhantomData,
	ops::*
};



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
			unsafe { let _ = Box::from_raw(data); }
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

	/// Deletes all cookies.
	/// If `url` is not an empty string, only the cookies of the given url will be deleted.
	pub async fn clear(&mut self, url: &str) -> usize {
		self.delete(url, "").await
	}

	/// Like `clear`, but with `url` set empty.
	pub async fn clear_all(&mut self) -> usize {
		self.clear("").await
	}

	fn _delete<H>(&mut self, url: &str, name: &str, on_complete: H) where
		H: FnOnce(usize)
	{
		let data = Box::into_raw(Box::new(on_complete));

		self.inner.delete(url, name, cookie_delete_callback::<H>, data as _);
	}

	/// Deletes all cookies with the given `name`.
	/// If `url` is not empty, only the cookie with the given `name` at that `url` will be deleted.
	/// If `name` is empty, all cookies at the given `url` will be deleted.
	/// If both `url` and `name` are empty, all cookies will be deleted.
	pub async fn delete(&mut self, url: &str, name: &str) -> usize {
		let (tx, rx) = oneshot::channel::<usize>();

		self._delete(url, name, |result| {
			tx.send(result).expect("unable to send back cookie delete count");
		});

		rx.await.unwrap()
	}

	/// Like `delete`, but with `url` set empty.
	pub async fn delete_all(&mut self, name: &str) -> usize {
		self.delete("", name).await
	}

	/// Finds the first cookie that has the given `name` in the given `url`.
	/// If `include_http_only` is set to `false`, a `HttpOnly` cookie will not be found.
	pub async fn find(&self, url: &str, name: &str, include_http_only: bool) -> Option<Cookie> {
		let mut iter = self.iter(url, include_http_only);

		while let Some(cookie) = iter.next().await {
			if cookie.name() == name {
				return Some(cookie)
			}
		}

		None
	}

	/// Finds the first cookie that has the given `name`.
	pub async fn find_from_all(&self, name: &str) -> Option<Cookie> {
		let mut iter = self.iter_all();

		while let Some(cookie) = iter.next().await {
			if cookie.name() == name {
				return Some(cookie)
			}
		}

		None
	}

	pub(in crate) fn global() -> Option<Self> {
		Some(Self {
			inner: CookieJarImpl::global()
		})
	}

	/// Returns a `CookieIterator` that iterates over cookies asynchronously.
	/// The `CookieIterator` has an async `next` function that you can use.
	/// 
	/// # Example
	/// ```ignore
	/// let cookie_jar = app.cookie_jar();
	/// let mut iterator = cookie_jar.iter("http://localhost/", true);
	/// 
	/// while let Some(cookie) = iterator.next().await {
	/// 	// ... do something with `cookie`
	/// }
	/// ```
	pub fn iter<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIterator<'a> {
		let inner = self.inner.iterator(url, include_http_only);

		CookieIterator {
			inner,
			_phantom: PhantomData
		}
	}

	/// Returns a `CookieIterator` that iterators over cookies asynchronously.
	/// Like `iter`, but iterates over all cookies from any url.
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

	/// Stores the given `cookie` for the given `url`.
	pub async fn store(&mut self, url: &str, cookie: &Cookie) -> Result<(), CookieStorageError> {
		let (tx, rx) = oneshot::channel::<Result<(), CookieStorageError>>();

		self._store(url, cookie, |result| {
			tx.send(result).expect("unable to retrieve cookie storage error");
		});

		rx.await.unwrap()
	}
}

impl Drop for CookieJar {
	fn drop(&mut self) {
		self.inner.free();
	}
}



unsafe fn cookie_delete_callback<'a, H>(_handle: CookieJarImpl, cb_data: *mut (), deleted: usize) where
	H: FnOnce(usize) + 'a
{
	let data_ptr = cb_data as *mut H;
	let data: Box<H> = Box::from_raw( data_ptr );

	(*data)( deleted );
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