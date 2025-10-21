use std::{
	boxed::Box,
	future::Future,
	mem, panic,
	panic::{AssertUnwindSafe, catch_unwind},
	pin::Pin,
	task::{Context, Poll, Waker},
};

use crate::{HasHandle, application::ApplicationHandle, core::application::*};

/// The data that is sent to the GUI thread for `DelegateFuture`.
struct DelegateData<'a, 'b, O, H, R> {
	handle: O,
	result: &'a mut Option<Result<R, DelegateError>>,
	func: Box<dyn FnOnce(&H) -> R + Send + 'b>,
	waker: Waker,
}
//unsafe impl<'a,H,R> Send for DelegateData<'a,H,R> {}

/// The data that is sent to the GUI thread for `DelegateFutureFuture`.
struct DelegateFutureData<'a, 'b, R>
where
	R: Send,
{
	inner: &'b mut DelegateFutureInner<'a, R>,
	waker: Waker,
}

/// The error that occurs when you're delegating work to the GUI thread, but it
/// fails to finish and/or return a result.
#[derive(Debug)]
pub enum DelegateError {
	/// The runtime has either not yet started or already ended.
	/// This happens when the application has already exited for example.
	RuntimeNotAvailable,
	/// The delegated closure has panicked.
	ClosurePanicked,
}

/// This future executes a closure on the GUI thread and returns the result.
pub struct DelegateFuture<'a, O, H, R>
where
	R: Send,
{
	handle: O,
	func: Option<Box<dyn FnOnce(&H) -> R + Send + 'a>>,
	result: Option<Result<R, DelegateError>>,
	started: bool,
}
impl<'a, O, H, R> Unpin for DelegateFuture<'a, O, H, R> where R: Send {}
/// # Safety
/// `DelegateFuture` by itself is not send.
/// This is because we keep a handle `H`, which is not necessarily `Send`.
/// However, because the closure only executes on the GUI thread,
///  and because the handle is only provided to the closure that will be
/// executed on the GUI thread,  this should be fine.
unsafe impl<'a, O, H, R> Send for DelegateFuture<'a, O, H, R> where R: Send {}

/// This future runs a future on the GUI thread and returns its output.
pub struct DelegateFutureFuture<'a, R>
where
	R: Send,
{
	app_handle: ApplicationHandle,
	inner: DelegateFutureInner<'a, R>,
	started: bool,
}
/// # Safety
/// `DelegateFutureFuture` by itself is not send.
/// This is because of `ApplicationHandle`.
/// However, because the closure only executes on the GUI thread,
///  and because this handle that is provided to the closure is something that
/// will only be sent with the closure to the GUI thread,  this should be fine.
unsafe impl<'a, R> Send for DelegateFutureFuture<'a, R> where R: Send {}
impl<'a, R> Unpin for DelegateFutureFuture<'a, R> where R: Send {}

/// This is not a future but the inner part that of `DelegateFutureFuture` that
/// needs to have mutable reference.
struct DelegateFutureInner<'a, R>
where
	R: Send,
{
	result: Option<Result<R, DelegateError>>,
	future: Pin<Box<dyn Future<Output = R> + 'a>>,
}
// # Safety
// `DelegateFutureInner` is marked as `Send` so that `delegate_async` can pass a
// non-`Send` future into this future. The resulting future from the closure of
// `delegate_async` does not need to be `Send` because the future is obtained
// _and_ executed within the GUI thread. `delegate_async` puts a future obtained
// from an `async` block into `DelegateFutureFuture`, and therefor in
// `DelegateFutureInner`. However, because the future obtained from the closure
// is not necessarily `Send`, Rust makes the whole async block non-`Send`.
// Even though all parts of that `async` block are executed on the same thread
// in this scenario. This is therefor marked as `Send` on the condition that
// whenever `DelegateFutureFuture` is constructed,  care should be taken to make
// sure that the future is safe to send to other threads.
unsafe impl<'a, R> Send for DelegateFutureInner<'a, R> where R: Send {}

#[cfg(feature = "threadsafe")]
impl<'a, O, H, R> DelegateFuture<'a, O, H, R>
where
	R: Send,
{
	pub(super) fn new<F>(handle: O, func: F) -> Self
	where
		F: FnOnce(&H) -> R + Send + 'a,
		R: Send,
	{
		Self {
			handle,
			func: Some(Box::new(func)),
			result: None,
			started: false,
		}
	}
}

#[cfg(feature = "threadsafe")]
impl<'a, O, H, R> Future for DelegateFuture<'a, O, H, R>
where
	O: HasHandle<H> + HasHandle<ApplicationHandle> + Clone,
	H: 'static,
	R: Send + 'static,
{
	type Output = Result<R, DelegateError>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		if !self.started {
			self.started = true;
			let app_inner = HasHandle::<ApplicationHandle>::handle(&self.handle)
				.inner
				.clone();

			// Move ownership from `DelegateFuture` to `DelegateData`
			let mut func = None;
			mem::swap(&mut self.func, &mut func);

			// Data to provide for the dispatched c function
			// This includes the closure to actually call,
			// a pointer to set the output with,
			// and a waker to finish our future with.
			let data = DelegateData {
				handle: self.handle.clone(),
				func: func.unwrap(),
				result: &mut self.result,
				waker: cx.waker().clone(),
			};

			let succeeded = {
				let data_ptr = Box::into_raw(Box::new(data));

				app_inner.dispatch(delegate_handler::<O, H, R>, data_ptr as _)
			};

			// cbw_Application_dispatch fails when there is now runtime that is running
			if !succeeded {
				return Poll::Ready(Err(DelegateError::RuntimeNotAvailable));
			}

			Poll::Pending
		} else {
			if self.result.is_none() {
				return Poll::Pending;
			}

			// Move ownership of output to temporary value so we can return it
			let mut temp: Option<Result<R, DelegateError>> = None;
			mem::swap(&mut self.result, &mut temp);

			Poll::Ready(temp.unwrap())
		}
	}
}

#[cfg(feature = "threadsafe")]
impl<'a, R> DelegateFutureFuture<'a, R>
where
	R: Send,
{
	pub(super) fn new(app_handle: ApplicationHandle, future: impl Future<Output = R> + 'a) -> Self {
		Self {
			app_handle,
			inner: DelegateFutureInner {
				result: None,
				future: Box::pin(future),
			},
			started: false,
		}
	}
}

#[cfg(feature = "threadsafe")]
impl<'a, R> Future for DelegateFutureFuture<'a, R>
where
	R: Send,
{
	type Output = Result<R, DelegateError>;

	fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
		// While the result is not yet set, we can keep polling
		if !self.started {
			self.started = true;
			let app_inner = self.app_handle.inner.clone();

			let data_ptr = Box::into_raw(Box::new(DelegateFutureData {
				inner: &mut self.inner,
				waker: ctx.waker().clone(),
			}));

			let succeeded = app_inner.dispatch(delegate_async_handler::<R>, data_ptr as _);

			// cbw_Application_dispatch fails when there is no runtime that is actually
			// running
			if !succeeded {
				return Poll::Ready(Err(DelegateError::RuntimeNotAvailable));
			}

			Poll::Pending
		} else {
			// Move ownership of output to temporary value so we can return it
			let mut temp: Option<Result<R, DelegateError>> = None;
			mem::swap(&mut self.inner.result, &mut temp);

			Poll::Ready(temp.unwrap())
		}
	}
}

fn delegate_handler<O, H, R>(app: ApplicationImpl, _data: *mut ())
where
	H: 'static,
	O: HasHandle<H>,
	R: 'static,
{
	let data_ptr = _data as *mut DelegateData<'static, 'static, O, H, R>;
	let data = unsafe { Box::from_raw(data_ptr) }; // Take ownership of the data struct

	match *data {
		DelegateData {
			handle,
			func,
			result,
			waker,
		} => {
			// Catch Rust panics during execution of delegated function
			match catch_unwind(AssertUnwindSafe(|| {
				let h = handle.handle();
				*result = Some(Ok(func(h)));
				waker.clone().wake();
			})) {
				Ok(()) => {}
				Err(_) => {
					*result = Some(Err(DelegateError::ClosurePanicked));

					// Wake the future before exiting. This allows the calling thread to still
					// receive the `DelegateError` before the application stops working.
					waker.wake();

					app.exit(-1);
				}
			}
		}
	}
}

#[cfg(feature = "threadsafe")]
fn delegate_async_handler<R>(app: ApplicationImpl, _data: *mut ())
where
	R: Send,
{
	let data_ptr = _data as *mut DelegateFutureData<R>;
	let data = unsafe { Box::from_raw(data_ptr) }; // Take ownership of the data struct

	match *data {
		DelegateFutureData { inner, waker } => {
			// Catch Rust panics
			match panic::catch_unwind(AssertUnwindSafe(|| {
				let mut ctx = Context::from_waker(&waker);
				match inner.future.as_mut().poll(&mut ctx) {
					Poll::Pending => {}
					Poll::Ready(result) => {
						// Set the result and wake our future so it gets returned
						inner.result = Some(Ok(result));
						waker.clone().wake();
					}
				}
			})) {
				Ok(()) => {}
				Err(_) => {
					inner.result = Some(Err(DelegateError::ClosurePanicked));

					// Wake the future before exiting. This allows the calling thread to still
					// receive the `DelegateError` before the application stops working.
					waker.wake();

					app.exit(-1);
				}
			}
		}
	}
}
