//! This module contains all event related types.

use std::{boxed::Box, future::Future, pin::Pin};

#[cfg(not(feature = "threadsafe"))]
pub type EventHandlerCallback<'a, H, A> = dyn FnMut(&H, &A) -> Pin<Box<dyn Future<Output = ()> + 'a>> + 'a;
pub type EventHandler<'a, H, A> = Box<EventHandlerCallback<'a, H, A>>;
#[cfg(feature = "threadsafe")]
pub type EventHandler<'a, A> = Box<dyn FnMut(&A) -> Pin<Box<dyn Future<Output = ()> + 'a>> + Send + 'a>;


/// An `Event` can be registered to with a regular closure or an 'async enclosure'.
/// All events are implemented for CEF.
/// If an event is not implemented for another browser framework, it will simply
/// never be invoked. If an event _is_ supported by another browser framework,
/// it should say so in its documentation.
pub(crate) trait Event<'a, H, A> {
	fn register_handler(&mut self, handler: EventHandler<'a, H, A>);
}

pub trait EventExt<'a, H, A> {
	/// Register a closure to be invoked for this event.
	#[cfg(not(feature = "threadsafe"))]
	fn register<X>(&mut self, handler: X)
	where
		X: FnMut(&H, &A) + 'a;

	/// Register a closure to be invoked for this event.
	#[cfg(feature = "threadsafe")]
	fn register<H>(&mut self, mut handler: H)
	where
		X: FnMut(&H, &A) + Send + 'a;

	/// Register an 'async closure' to be invoked for this event.
	///
	/// # Example
	/// ```ignore
	/// my_event.register_async(|args| async move {
	///     // Do something ...
	/// });
	/// ```
	#[cfg(not(feature = "threadsafe"))]
	fn register_async<X, F>(&mut self, handler: X)
	where
		X: FnMut(&H, &A) -> F + 'a,
		F: Future<Output = ()> + 'a;

	/// Register an 'async closure' to be invoked for this event.
	///
	/// # Example
	/// ```ignore
	/// my_event.register_async(|args| async move {
	///     // Do something ...
	/// });
	/// ```
	#[cfg(feature = "threadsafe")]
	fn register_async<X, F>(&mut self, mut handler: HX)
	where
		X: FnMut(&H, &A) -> F + Send + 'a,
		F: Future<Output = ()> + 'a;
}


impl<'a, H, A, T> EventExt<'a, H, A> for T where T: Event<'a, H, A> {
	#[cfg(not(feature = "threadsafe"))]
	fn register<X>(&mut self, mut handler: X)
	where
		X: FnMut(&H, &A) + 'a,
	{
		self.register_handler(Box::new(move |h, args| {
			handler(h, args);
			Box::pin(async {})
		}));
	}

	#[cfg(feature = "threadsafe")]
	fn register<X>(&mut self, mut handler: X)
	where
		X: FnMut(&H, &A) + Send + 'a,
	{
		self.register_handler(Box::new(move |h, args| {
			handler(h, args);
			Box::pin(async {})
		}));
	}

	#[cfg(not(feature = "threadsafe"))]
	fn register_async<X, F>(&mut self, mut handler: X)
	where
		X: FnMut(&H, &A) -> F + 'a,
		F: Future<Output = ()> + 'a,
	{
		self.register_handler(Box::new(move |h, args| Box::pin(handler(h, args))));
	}

	#[cfg(feature = "threadsafe")]
	fn register_async<X, F>(&mut self, mut handler: X)
	where
		X: FnMut(&H, &A) -> F + Send + 'a,
		F: Future<Output = ()> + 'a,
	{
		self.register_handler(Box::new(move |h, args| Box::pin(handler(h, args))));
	}
}


#[doc(hidden)]
#[macro_export]
macro_rules! decl_event {
	( $name:ident ) => {
		pub struct $name<'a> {
			pub(crate) handle: &'a BrowserWindowHandle,
		}

		impl<'a> $name<'a> {
			pub(crate) fn new(handle: &'a BrowserWindowHandle) -> Self {
				Self {
					handle,
				}
			}
		}
	}
}
#[doc(hidden)]
#[macro_export]
macro_rules! def_event {
	( $name:ident<$handle_type:ty, $arg_type:ty> (&mut $this:ident, $arg_name:ident) $body:block ) => {
		impl<'a> crate::event::Event<'a, $handle_type, $arg_type> for $name<'a> {
			fn register_handler(&mut $this, $arg_name: crate::event::EventHandler<'a, $handle_type, $arg_type>) $body
		}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! def_browser_event {
	( $name:ident<$arg_type:ty> (&mut $this:ident, $arg_name:ident) $body:block ) => {
		def_event!($name<BrowserWindowHandle, $arg_type> (&mut $this, $arg_name) $body);
	}
}