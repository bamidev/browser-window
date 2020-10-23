use super::application::{ApplicationHandle};
use boxfnonce::SendBoxFnOnce;
use browser_window_ffi::*;
use std::future::Future;
use std::mem;
use std::os::raw::*;
use std::pin::Pin;
use std::task::{
	Context,
	Poll,
	Waker
};



pub struct DispatchData<'a,H,R> {

	handle: H,
	func: Option<SendBoxFnOnce<'a,( H, ),R>>,
	result_ptr: *mut Option<Box<R>>,
	waker: Waker
}
unsafe impl<'a,H,R> Send for DispatchData<'a,H,R> {}

pub struct DispatchFuture<'a,H,R> {

	handle: H,
	func: Option<SendBoxFnOnce<'a,( H, ),R>>,
	result: Option<Box<R>>,
	started: bool
}
impl<'a,H,R> Unpin for DispatchFuture<'a,H,R> {}
//unsafe impl<'a,H,R> Send for DispatchFuture<'a,H,R> {}

impl<'a,H,R> DispatchFuture<'a,H,R> {

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

impl<'a,H,R> Future for DispatchFuture<'a,H,R> where
	H: AppHandle + Clone
	//R: Send
{
	type Output = Box<R>;

	fn poll( mut self: Pin<&mut Self>, cx: &mut Context ) -> Poll<Self::Output> {

		if !self.started {

			// Data to provide for the dispatched c function
			// This includes the closure to actually call,
			// a pointer to set the output with,
			// and a waker to finish our future with.
			let mut data = Box::new( DispatchData {
				handle: self.handle.clone(),
				func: None,
				result_ptr: unsafe { mem::transmute( &self.result ) },
				waker: cx.waker().clone()
			} );
			// Move ownership of the boxed FnOnce to the data struct
			// Our future doesn't need it itself
			mem::swap( &mut self.func, &mut data.func );

			unsafe {
				let data_ptr = Box::into_raw( data );

				bw_Application_dispatch(
					self.handle.app_handle()._ffi_handle,
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
			let mut temp: Option<Box<R>> = None;
			mem::swap( &mut self.result, &mut temp );

			Poll::Ready( temp.unwrap() )
		}
	}
}



// The trait to be implemented by thread-unsafe handles
pub trait AppHandle {
	fn app_handle( &self ) -> ApplicationHandle;
}



extern "C" fn ffi_dispatch_handler<H,R>( _app: *mut bw_Application, _data: *mut c_void ) where
	H: Clone
{

	unsafe {
		let data_ptr: *mut DispatchData<H,R> = mem::transmute( _data );
		let data = Box::from_raw( data_ptr );	// Take ownership of the data struct

		match *data {
			DispatchData{ handle, func, result_ptr, waker } => {
				let handle: H = handle.clone();

				let result = func.unwrap().call( handle );
				*result_ptr = Some( Box::new( result ) );
				waker.wake();	// Notify that the result value has been set
			}
		}
	}
}
