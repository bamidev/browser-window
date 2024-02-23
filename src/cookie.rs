//! Module for dealing with cookies.

use std::{borrow::Cow, marker::PhantomData, ops::*, time::SystemTime};

use futures_channel::oneshot;

use crate::core::cookie::*;

pub struct Cookie(CookieImpl);

pub struct CookieJar(CookieJarImpl);

pub struct CookieIterator<'a> {
	inner: CookieIteratorImpl,
	_phantom: PhantomData<&'a u8>,
}

impl Cookie {
	pub fn new(name: &str, value: &str) -> Self {
		Self(CookieImpl::new(name, value))
	}

	pub fn creation_time(&self) -> SystemTime {
		self.0.creation_time()
	}

	pub fn expires(&self) -> Option<SystemTime> {
		self.0.expires()
	}

	pub fn domain(&self) -> Cow<'_, str> {
		self.0.domain()
	}

	pub fn is_http_only(&self) -> bool {
		self.0.is_http_only()
	}

	pub fn name(&self) -> Cow<'_, str> {
		self.0.name()
	}

	pub fn path(&self) -> Cow<'_, str> {
		self.0.path()
	}

	pub fn is_secure(&self) -> bool {
		self.0.is_secure()
	}

	pub fn value(&self) -> Cow<'_, str> {
		self.0.value()
	}

	pub fn make_http_only(&mut self) -> &mut Self {
		self.0.make_http_only();
		self
	}

	pub fn make_secure(&mut self) -> &mut Self {
		self.0.make_secure();
		self
	}

	pub fn set_creation_time(&mut self, time: &SystemTime) -> &mut Self {
		self.0.set_creation_time(time);
		self
	}

	pub fn set_expires(&mut self, time: &SystemTime) -> &mut Self {
		self.0.set_expires(time);
		self
	}

	pub fn set_domain(&mut self, domain: &str) -> &mut Self {
		self.0.set_domain(domain);
		self
	}

	pub fn set_name(&mut self, name: &str) -> &mut Self {
		self.0.set_name(name);
		self
	}

	pub fn set_path(&mut self, path: &str) -> &mut Self {
		self.0.set_path(path);
		self
	}

	pub fn set_value(&mut self, value: &str) -> &mut Self {
		self.0.set_value(value);
		self
	}
}

impl Drop for Cookie {
	fn drop(&mut self) {
		self.0.free();
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

	fn _next<H>(&mut self, on_next: H) -> bool
	where
		H: FnOnce(Option<Cookie>),
	{
		let data = Box::into_raw(Box::new(on_next));

		let called_closure = self
			.inner
			.next(cookie_iterator_next_handler::<H>, data as _);

		if !called_closure {
			unsafe {
				let _ = Box::from_raw(data);
			}
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
	/// If `url` is not an empty string, only the cookies of the given url will
	/// be deleted.
	pub async fn clear(&mut self, url: &str) -> usize {
		self.delete(url, "").await
	}

	/// Like `clear`, but with `url` set empty.
	pub async fn clear_all(&mut self) -> usize {
		self.clear("").await
	}

	fn _delete<H>(&mut self, url: &str, name: &str, on_complete: H)
	where
		H: FnOnce(usize),
	{
		let data = Box::into_raw(Box::new(on_complete));

		self.0
			.delete(url, name, cookie_delete_callback::<H>, data as _);
	}

	/// Deletes all cookies with the given `name`.
	/// If `url` is not empty, only the cookie with the given `name` at that
	/// `url` will be deleted. If `name` is empty, all cookies at the given
	/// `url` will be deleted. If both `url` and `name` are empty, all cookies
	/// will be deleted.
	pub async fn delete(&mut self, url: &str, name: &str) -> usize {
		let (tx, rx) = oneshot::channel::<usize>();

		self._delete(url, name, |result| {
			tx.send(result)
				.expect("unable to send back cookie delete count");
		});

		rx.await.unwrap()
	}

	/// Like `delete`, but with `url` set empty.
	pub async fn delete_all(&mut self, name: &str) -> usize {
		self.delete("", name).await
	}

	/// Finds the first cookie that has the given `name` in the given `url`.
	/// If `include_http_only` is set to `false`, a `HttpOnly` cookie will not
	/// be found.
	pub async fn find(&self, url: &str, name: &str, include_http_only: bool) -> Option<Cookie> {
		let mut iter = self.iter(url, include_http_only);

		while let Some(cookie) = iter.next().await {
			if cookie.name() == name {
				return Some(cookie);
			}
		}

		None
	}

	/// Finds the first cookie that has the given `name`.
	pub async fn find_from_all(&self, name: &str) -> Option<Cookie> {
		let mut iter = self.iter_all();

		while let Some(cookie) = iter.next().await {
			if cookie.name() == name {
				return Some(cookie);
			}
		}

		None
	}

	pub(crate) fn global() -> Option<Self> {
		CookieJarImpl::global().map(|i| Self(i))
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
		let inner = self.0.iterator(url, include_http_only);

		CookieIterator {
			inner,
			_phantom: PhantomData,
		}
	}

	/// Returns a `CookieIterator` that iterators over cookies asynchronously.
	/// Like `iter`, but iterates over all cookies from any url.
	pub fn iter_all<'a>(&'a self) -> CookieIterator<'a> {
		let inner = self.0.iterator_all();

		CookieIterator {
			inner,
			_phantom: PhantomData,
		}
	}

	fn _store<'a, H>(&mut self, url: &str, cookie: &Cookie, on_complete: H)
	where
		H: FnOnce(Result<(), CookieStorageError>) + 'a,
	{
		let data = Box::into_raw(Box::new(on_complete));

		self.0.store(
			url.into(),
			&cookie.0,
			Some(cookie_store_callback::<'a, H>),
			data as _,
		);
	}

	/// Stores the given `cookie` for the given `url`.
	pub async fn store(&mut self, url: &str, cookie: &Cookie) -> Result<(), CookieStorageError> {
		let (tx, rx) = oneshot::channel::<Result<(), CookieStorageError>>();

		self._store(url, cookie, |result| {
			tx.send(result)
				.expect("unable to retrieve cookie storage error");
		});

		rx.await.unwrap()
	}
}

impl Drop for CookieJar {
	fn drop(&mut self) {
		self.0.free();
	}
}

unsafe fn cookie_delete_callback<'a, H>(_handle: CookieJarImpl, cb_data: *mut (), deleted: usize)
where
	H: FnOnce(usize) + 'a,
{
	let data_ptr = cb_data as *mut H;
	let data: Box<H> = Box::from_raw(data_ptr);

	(*data)(deleted);
}

unsafe fn cookie_store_callback<'a, H>(
	_handle: CookieJarImpl, cb_data: *mut (), result: Result<(), CookieStorageError>,
) where
	H: FnOnce(Result<(), CookieStorageError>) + 'a,
{
	let data_ptr = cb_data as *mut H;
	let data: Box<H> = Box::from_raw(data_ptr);

	(*data)(result);
}

unsafe fn cookie_iterator_next_handler<H>(
	_handle: CookieIteratorImpl, cb_data: *mut (), cookie: Option<CookieImpl>,
) where
	H: FnOnce(Option<Cookie>),
{
	let data_ptr = cb_data as *mut H;
	let data: Box<H> = Box::from_raw(data_ptr);

	(*data)(cookie.map(|c| Cookie(c)));
}
