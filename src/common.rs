use super::application::ApplicationHandle;
use browser_window_ffi::*;
use std::boxed::Box;
use std::future::Future;
use std::mem;
use std::os::raw::*;
use std::panic;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{
	Context,
	Poll,
	Waker
};
use std::panic::{AssertUnwindSafe, catch_unwind};



/// The data that is sent to the GUI thread for `DelegateAsyncFuture`.
/*struct DelegateAsyncData<'a,'b,H,F,R> {
	handle: H,
	result: &'a mut Option<Result<R, DelegateError>>,
	func: Option<Box<dyn FnOnce( H ) -> F + Send + 'b>>,
	waker: Waker
}*/

/// The data that is sent to the GUI thread for `DelegateFuture`.
struct DelegateData<'a,'b,H,R> {
	handle: H,
	result: &'a mut Option<Result<R, DelegateError>>,
	func: Box<dyn FnOnce( H ) -> R + Send + 'b>,
	waker: Waker
}
//unsafe impl<'a,H,R> Send for DelegateData<'a,H,R> {}

/// The data that is sent to the GUI thread for `DelegateFutureFuture`.
struct DelegateFutureData<'a,'b,R> where R: Send{
	inner: &'b mut DelegateFutureInner<'a,R>,
	waker: Waker
}

#[derive(Debug)]
pub enum DelegateError {
	/// The runtime has either not yet started or already ended.
	/// This happens when the application has exited.
	RuntimeNotAvailable,
	/// The closure that has been delegated panicked.
	ClosurePanicked
}

/// This future executes a async closure on the GUI thread and returns the result.
/*pub struct DelegateAsyncFuture<'a,H,F,R> {
	handle: H,
	func: Option<Box<dyn FnOnce( H ) -> F + Send + 'a>>,
	fut: Option<Pin<Box<dyn Future<Output=R>>>>,
	result: Option<Result<R, DelegateError>>
}
impl<'a,H,G,R> Unpin for DelegateAsyncFuture<'a,H,G,R> {}*/

/// This future executes a closure on the GUI thread and returns the result.
pub struct DelegateFuture<'a,H,R> where
	R: Send
{
	handle: H,
	func: Option<Box<dyn FnOnce( H ) -> R + Send + 'a>>,
	result: Option<Result<R, DelegateError>>,
	started: bool
}
impl<'a,H,R> Unpin for DelegateFuture<'a,H,R> where R: Send {}

/// This future runs a future on the GUI thread and returns its output.
pub struct DelegateFutureFuture<'a,R> where
	R: Send
{
	app_handle: ApplicationHandle,
	inner: DelegateFutureInner<'a,R>,
	started: bool
}
impl<'a,R> Unpin for DelegateFutureFuture<'a,R> where R: Send {}

/// This is not a future but the inner part that of `DelegateFutureFuture` that needs to have mutable reference.
struct DelegateFutureInner<'a,R> where R: Send {
	result: Option<Result<R, DelegateError>>,
	future: Pin<Box<dyn Future<Output=R> + 'a>>
}
// # Safety
// `DelegateFutureInner` is marked as `Send` so that `delegate_async` can pass a non-`Send` future into this future.
// The resulting future from the closure of `delegate_async` does not need to be `Send` because the future is obtained _and_ executed within the GUI thread.
// `delegate_async` puts a future obtained from an `async` block into `DelegateFutureFuture`, and therefor in `DelegateFutureInner`.
// However, because the future obtained from the closure is not necessarily `Send`, Rust makes the whole async block non-`Send`.
// Even though all parts of that `async` block are executed on the same thread in this scenario.
// This is therefor marked as `Send` on the condition that whenever `DelegateFutureFuture` is constructed,
//  care should be taken to make sure that the future is safe to send to other threads.
unsafe impl<'a,R> Send for DelegateFutureInner<'a,R> where R: Send {}



impl<'a,H,R> DelegateFuture<'a,H,R> where R: Send {

	pub(in super) fn new<F>( handle: H, func: F ) -> Self where
		F: FnOnce( H ) -> R + Send + 'a,
		R: Send
	{
		Self {
			handle,
			func: Some( Box::new( func ) ),
			result: None,
			started: false
		}
	}
}

impl<'a,H,R> Future for DelegateFuture<'a,H,R> where
	H: HasAppHandle + Clone + 'static,
	R: Send + 'static
{
	type Output = Result<R, DelegateError>;

	fn poll( mut self: Pin<&mut Self>, cx: &mut Context ) -> Poll<Self::Output> {

		if !self.started {
			self.started = true;
			let app_ffi_handle = self.handle.app_handle().ffi_handle;

			// Move ownership from `DelegateFuture` to `DelegateData`
			let mut func = None;
			mem::swap( &mut self.func, &mut func );

			// Data to provide for the dispatched c function
			// This includes the closure to actually call,
			// a pointer to set the output with,
			// and a waker to finish our future with.
			let data = DelegateData {
				handle: self.handle.clone(),
				func: func.unwrap(),
				result: &mut self.result,
				waker: cx.waker().clone()
			};

			let succeeded = unsafe {
				let data_ptr = Box::into_raw( Box::new( data ) );

				bw_Application_dispatch(
					app_ffi_handle,
					ffi_delegate_handler::<H,R>,
					data_ptr as _
				)
			};

			// bw_Application_dispatch fails when there is now runtime that is running
			if !succeeded {
				return Poll::Ready( Err( DelegateError::RuntimeNotAvailable ) );
			}

			Poll::Pending
		}
		else {
			if self.result.is_none() {
				return Poll::Pending;
			}

			// Move ownership of output to temporary value so we can return it
			let mut temp: Option<Result<R, DelegateError>> = None;
			mem::swap( &mut self.result, &mut temp );

			Poll::Ready( temp.unwrap() )
		}
	}
}

impl<'a,R> DelegateFutureFuture<'a,R> where R: Send {

	pub(in super) fn new( app_handle: ApplicationHandle, future: impl Future<Output=R> + 'a ) -> Self {
		Self {
			app_handle,
			inner: DelegateFutureInner {
				result: None,
				future: Box::pin( future )
			},
			started: false
		}
	}
}

impl<'a,R> Future for DelegateFutureFuture<'a,R> where R: Send {
	type Output = Result<R, DelegateError>;

	fn poll( mut self: Pin<&mut Self>, ctx: &mut Context ) -> Poll<Self::Output> {

		// While the result is not yet set, we can keep polling
		if !self.started {
			self.started = true;
			let app_ffi_handle = self.app_handle.ffi_handle;

			let succeeded = unsafe {

				let data_ptr = Box::into_raw(Box::new(DelegateFutureData {
					inner: &mut self.inner,
					waker: ctx.waker().clone(),
				}));

				bw_Application_dispatch(
					app_ffi_handle,
					ffi_delegate_async_handler::<R>,
					data_ptr as _
				)
			};

			// bw_Application_dispatch fails when there is now runtime that is running
			if !succeeded {
				return Poll::Ready(Err(DelegateError::RuntimeNotAvailable));
			}

			Poll::Pending
		}
		else {
			// Move ownership of output to temporary value so we can return it
			let mut temp: Option<Result<R, DelegateError>> = None;
			mem::swap( &mut self.inner.result, &mut temp );

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




extern "C" fn ffi_delegate_handler<H,R>( app: *mut bw_Application, _data: *mut c_void ) where
	H: Clone + 'static,
	R: 'static
{

	unsafe {
		let data_ptr: *mut DelegateData<'static,'static,H,R> = mem::transmute( _data );
		let data = Box::from_raw( data_ptr );	// Take ownership of the data struct

		match *data {
			DelegateData{ handle, func, result, waker } => {

				match catch_unwind(AssertUnwindSafe(|| {
					*result = Some( Ok( func( handle ) ) );
					waker.clone().wake();
				})) {
					Ok(()) => {},
					Err( _ ) => {
						*result = Some(Err(DelegateError::ClosurePanicked));

						// Wake the future before exiting. This allows the calling thread to still receive the `DelegateError` before the application stops working.
						waker.wake();

						bw_Application_exit( app, -1 );
					}
				}
			}
		}
	}
}

extern "C" fn ffi_delegate_async_handler<R>( app: *mut bw_Application, _data: *mut c_void ) where R: Send {

	unsafe {
		let data_ptr: *mut DelegateFutureData<R> = mem::transmute( _data );
		let data = Box::from_raw( data_ptr );	// Take ownership of the data struct

		match *data {
			DelegateFutureData{ inner, waker } => {

				match panic::catch_unwind(AssertUnwindSafe(|| {

					let mut ctx  = Context::from_waker( &waker );
					match inner.future.as_mut().poll( &mut ctx ) {
						Poll::Pending => {},
						Poll::Ready( result) => {
							// Set the result and wake our future so it gets returned
							inner.result = Some( Ok( result ) );
							waker.clone().wake();
						}
					}
				})) {
					Ok(()) => {},
					Err( _ ) => {
						inner.result = Some(Err(DelegateError::ClosurePanicked));

						// Wake the future before exiting. This allows the calling thread to still receive the `DelegateError` before the application stops working.
						waker.wake();

						bw_Application_exit( app, -1 );
					}
				}
			}
		}
	}
}
