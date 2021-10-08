use super::{CookieExt, CookieJarExt, CookieIteratorExt};

use std::{
	borrow::Cow,
	marker::PhantomData,
	mem::MaybeUninit,
	ptr
};

use browser_window_c::*;



pub struct CookieImpl {
	pub(in crate) inner: *mut cbw_Cookie
}

pub struct CookieJarImpl {
	pub(in crate) inner: *mut cbw_CookieJar
}

pub struct CookieIteratorImpl<'a> {
	pub(in crate) inner: *mut cbw_CookieIterator,
	_phantom: PhantomData<&'a u8>
}



impl CookieExt for CookieImpl {
	fn name<'a>(&'a self) -> Cow<'a, str> {
		let slice = unsafe { cbw_Cookie_getName(self.inner) };

		let string: String = slice.into();
		return string.into()
	}

	fn value<'a>(&'a self) -> Cow<'a, str> {
		let slice = unsafe { cbw_Cookie_getValue(self.inner) };

		let string: String = slice.into();
		return string.into()
	}
}

impl Drop for CookieImpl {
	fn drop(&mut self) {
		unsafe { cbw_Cookie_free(self.inner) };
	}
}

impl CookieJarExt for CookieJarImpl {

	fn global() -> CookieJarImpl {
		let inner = unsafe { cbw_CookieJar_newGlobal() };

		CookieJarImpl {
			inner
		}
	}

	fn iterator<'a>(&'a self, url: &str, include_http_only: bool) -> CookieIteratorImpl<'a> {
		let mut iterator: CookieIteratorImpl<'a> = unsafe { MaybeUninit::uninit().assume_init() };
		unsafe { cbw_CookieJar_iterator(self.inner, &mut iterator.inner, if include_http_only {1} else {0}, url.into()) };
		
		return iterator;
	}
}

impl Drop for CookieJarImpl {
	fn drop(&mut self) {
		unsafe { cbw_CookieJar_free(self.inner) };
	}
}

impl<'a> CookieIteratorExt for CookieIteratorImpl<'a> {
	fn next(&mut self) -> Option<CookieImpl> {
		let mut cookie_ptr: *mut cbw_Cookie = ptr::null_mut();
		let success = unsafe { cbw_CookieIterator_next(self.inner, &mut cookie_ptr) };
		
		if success > 0 {
			Some(CookieImpl { inner: cookie_ptr })
		}
		else {
			None
		}
	}
}

impl<'a> Drop for CookieIteratorImpl<'a> {
	fn drop(&mut self) {
		unsafe { cbw_CookieIterator_free(self.inner) }
	}
}