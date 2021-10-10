use super::*;

use std::{
	borrow::Cow,
	ffi::c_void,
	ops::Add,
	marker::PhantomData,
	mem::MaybeUninit,
	ptr,
	time::{Duration, SystemTime}
};

use browser_window_c::*;



pub struct CookieImpl {
	pub(in crate) inner: *mut cbw_Cookie
}

pub struct CookieMutImpl (CookieImpl);

pub struct CookieJarImpl {
	pub(in crate) inner: *mut cbw_CookieJar
}

pub struct CookieIteratorImpl<'a> {
	pub(in crate) inner: *mut cbw_CookieIterator,
	_phantom: PhantomData<&'a u8>
}

struct CookieStorageCallbackData {
	callback: CookieStorageCallbackFn,
	data: *mut ()
}



impl CookieExt for CookieImpl {

	fn creation_time(&self) -> SystemTime {
		let timestamp = unsafe { cbw_Cookie_getCreationTime(self.inner) };

		SystemTime::UNIX_EPOCH.add(Duration::from_millis(timestamp))
	}

	fn expires(&self) -> Option<SystemTime> {
		let timestamp = unsafe { cbw_Cookie_getExpires(self.inner) };

		if timestamp != 0 {
			Some( SystemTime::UNIX_EPOCH.add(Duration::from_millis(timestamp)) )
		}
		else {
			None
		}
	}

	fn domain<'a>(&'a self) -> Cow<'a, str> {
		let mut slice: cbw_StrSlice = unsafe { MaybeUninit::uninit().assume_init() };
		let owned = unsafe { cbw_Cookie_getDomain(self.inner, &mut slice) };

		if owned > 0 {
			let string: String = slice.into();
			unsafe { cbw_string_free(slice) };
			string.into()
		}
		else {
			let string: &str = slice.into();
			string.into()
		}
	}

	fn is_http_only(&self) -> bool {
		(unsafe { cbw_Cookie_isHttpOnly(self.inner) }) > 0
	}

	fn is_secure(&self) -> bool {
		(unsafe { cbw_Cookie_isSecure(self.inner) }) > 0
	}

	fn name<'a>(&'a self) -> Cow<'a, str> {
		let mut slice: cbw_StrSlice = unsafe { MaybeUninit::uninit().assume_init() };
		let owned = unsafe { cbw_Cookie_getName(self.inner, &mut slice) };

		if owned > 0 {
			let string: String = slice.into();println!("name:{}",string);
			unsafe { cbw_string_free(slice) };println!("owned:{}",owned);
			string.into()
		}
		else {
			let string: &str = slice.into();
			string.into()
		}
	}

	fn new(name: &str, value: &str) -> Self {
		let inner = unsafe { cbw_Cookie_new(name.into(), value.into()) };

		Self { inner }
	}

	fn path<'a>(&'a self) -> Cow<'a, str> {
		let mut slice: cbw_StrSlice = unsafe { MaybeUninit::uninit().assume_init() };
		let owned = unsafe { cbw_Cookie_getPath(self.inner, &mut slice) };

		if owned > 0 {
			let string: String = slice.into();
			unsafe { cbw_string_free(slice) };
			string.into()
		}
		else {
			let string: &str = slice.into();
			string.into()
		}
	}

	fn value<'a>(&'a self) -> Cow<'a, str> {
		let mut slice: cbw_StrSlice = unsafe { MaybeUninit::uninit().assume_init() };

		let owned = unsafe { cbw_Cookie_getValue(self.inner, &mut slice) };
		
		if owned > 0 {
			let string: String = slice.into();
			unsafe { cbw_string_free(slice) };
			string.into()
		}
		else {
			let string: &str = slice.into();
			string.into()
		}
	}
	
	fn make_http_only(&mut self) -> &mut Self {
		unsafe { cbw_Cookie_makeHttpOnly(self.inner) };	self
	}

	fn make_secure(&mut self) -> &mut Self {
		unsafe { cbw_Cookie_makeSecure(self.inner) };	self
	}

	fn set_creation_time(&mut self, time: &SystemTime) -> &mut Self {
		unsafe { cbw_Cookie_setCreationTime(self.inner, time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as _) };	self
	}

	fn set_expires(&mut self, time: &SystemTime) -> &mut Self {
		unsafe { cbw_Cookie_setExpires(self.inner, time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as _) };	self
	}

	fn set_domain(&mut self, domain: &str) -> &mut Self {
		unsafe { cbw_Cookie_setDomain(self.inner, domain.into()) };	self
	}

	fn set_name(&mut self, name: &str) -> &mut Self {
		unsafe { cbw_Cookie_setName(self.inner, name.into()) };	self
	}


	fn set_path(&mut self, path: &str) -> &mut Self {
		unsafe { cbw_Cookie_setPath(self.inner, path.into()) };	self
	}

	fn set_value(&mut self, value: &str) -> &mut Self{
		unsafe { cbw_Cookie_setValue(self.inner, value.into()) };	self
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

	fn store(&self, url: &str, cookie: &CookieImpl, success_cb: Option<CookieStorageCallbackFn>, cb_data: *mut ()) {
		let data = if !success_cb.is_none() {
			Box::into_raw( Box::new( CookieStorageCallbackData {
				callback: success_cb.unwrap(),
				data: cb_data
			}))
		}
		else {
			ptr::null_mut()
		};

		unsafe { cbw_CookieJar_store(
			self.inner,
			url.into(),
			cookie.inner,
			success_cb.map(|_| ffi_cookie_storage_callback_handler as _),
			data as _
		) };
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



unsafe extern "C" fn ffi_cookie_storage_callback_handler(cookie_jar: *mut cbw_CookieJar, _data: *mut c_void, error: cbw_Err) {

	let data_ptr = _data as *mut CookieStorageCallbackData;
	let data: Box<CookieStorageCallbackData> = Box::from_raw( data_ptr );

	let handle = CookieJarImpl {inner: cookie_jar};
	let result = if error.code == 0 {
		Ok(())
	}
	else {
		Err(CookieStorageError::Unknown)
	};

	(data.callback)( handle, data.data, result );
}