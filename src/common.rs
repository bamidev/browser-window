use super::application::ApplicationHandle;
use boxfnonce::SendBoxFnOnce;
use browser_window_ffi::*;
use std::future::Future;
use std::mem;
use std::os::raw::*;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{
	Context,
	Poll,
	Waker
};



pub struct DelegateData<'a,H,R> {

	handle: H,
	func: Option<SendBoxFnOnce<'a,( H, ),R>>,
	result_ptr: *mut Option<R>,
	waker: Waker
}
unsafe impl<'a,H,R> Send for DelegateData<'a,H,R> {}

pub struct DelegateFuture<'a,H,R> {

	handle: H,
	func: Option<SendBoxFnOnce<'a,( H, ),R>>,
	result: Option<R>,
	started: bool
}
impl<'a,H,R> Unpin for DelegateFuture<'a,H,R> {}

impl<'a,H,R> DelegateFuture<'a,H,R> {

	pub fn new<F>( handle: H, func: F ) -> Self where
		F: FnOnce( H ) -> R + Send + 'a,
		R: Send
	{
		Self {
			handle: handle,
			func: Some( SendBoxFnOnce::new( func ) ),
			result: None,
			started: false
		}
	}
}

impl<'a,H,R> Future for DelegateFuture<'a,H,R> where
	H: HasAppHandle + Clone
{
	type Output = R;

	fn poll( mut self: Pin<&mut Self>, cx: &mut Context ) -> Poll<Self::Output> {

		if !self.started {

			// Data to provide for the dispatched c function
			// This includes the closure to actually call,
			// a pointer to set the output with,
			// and a waker to finish our future with.
			let mut data = Box::new( DelegateData::<H,R> {
				handle: self.handle.clone(),
				func: None,
				result_ptr: &mut self.result as _,
				waker: cx.waker().clone()
			} );

			// Move ownership of the boxed FnOnce to the data struct
			// Our future doesn't need it itself
			mem::swap( &mut self.func, &mut data.func );
			unsafe {
				let data_ptr = Box::into_raw( data );

				bw_Application_dispatch(
					self.handle.app_handle().ffi_handle,
					ffi_dispatch_handler::<H,R>,
					data_ptr as _
				);
			}

			self.started = true;
			Poll::Pending
		}
		else {
			if self.result.is_none() {
				return Poll::Pending;
			}

			// Move ownership of output to temporary value so we can return it
			let mut temp: Option<R> = None;
			mem::swap( &mut self.result, &mut temp );

			Poll::Ready( temp.unwrap() )
		}
	}
}



// The trait to be implemented by all (user-level) handles that are able to return an ApplicationHandle.
// Like: Application, ApplicationAsync, BrowserWindow, BrowserWindowAsync
pub trait HasAppHandle {
	fn app_handle( &self ) -> ApplicationHandle;
}



/// A trait that is able to tell whether or not the pointer-like type that implements it,
///  points to the same location as another instance of the pointer.
pub trait PointerEq {
	fn ptr_eq( &self, other: &Self ) -> bool;
}



impl<T> PointerEq for Rc<T> {
	fn ptr_eq( &self, other: &Self  ) -> bool {
		Rc::ptr_eq( self, other )
	}
}

impl<T> PointerEq for Arc<T> {
	fn ptr_eq( &self, other: &Self  ) -> bool {
		Arc::ptr_eq( self, other )
	}
}




extern "C" fn ffi_dispatch_handler<H,R>( _app: *mut bw_Application, _data: *mut c_void )
{
	unsafe {
		let data_ptr: *mut DelegateData<H,R> = mem::transmute( _data );
		let data = Box::from_raw( data_ptr );	// Take ownership of the data struct

		match *data {
			DelegateData{ handle, func, result_ptr, waker } => {

				let result = func.unwrap().call( handle );
				*result_ptr = Some( result );
				waker.wake();	// Notify that the result value has been set
			}
		}
	}
}
