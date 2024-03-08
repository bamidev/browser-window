//! This module contains all event related types.

use std::{boxed::Box, future::Future, pin::Pin};


#[cfg(not(feature = "threadsafe"))]
pub type EventHandlerAsyncCallback<O, A> =
	dyn FnMut(O, &A) -> Pin<Box<dyn Future<Output = ()> + 'static>> + 'static;
#[cfg(not(feature = "threadsafe"))]
pub type EventHandlerSyncCallback<H, A> = dyn FnMut(&H, &A) + 'static;
#[cfg(feature = "threadsafe")]
pub type EventHandlerAsyncCallback<H, A> =
	dyn FnMut(O, &A) -> Pin<Box<dyn Future<Output = ()> + 'static>> + Send + 'static;
#[cfg(feature = "threadsafe")]
pub type EventHandlerSyncCallback<O, A> = dyn FnMut(&H, &A) + Send + 'static;


pub enum EventHandler<H, O, A> {
	Sync(Box<EventHandlerSyncCallback<H, A>>),
	Async(Box<EventHandlerAsyncCallback<O, A>>),
}


/// An `Event` can be registered to with a regular closure or an 'async
/// enclosure'. All events are implemented for CEF.
/// If an event is not implemented for another browser framework, it will simply
/// never be invoked. If an event _is_ supported by another browser framework,
/// it should say so in its documentation.
pub(crate) trait Event<H, O, A> {
	fn register_handler(&mut self, handler: EventHandler<H, O, A>);
}

pub trait EventExt<H, O, A> {
	/// Register a closure to be invoked for this event.
	#[cfg(not(feature = "threadsafe"))]
	fn register<X>(&mut self, handler: X)
	where
		X: FnMut(&H, &A) + 'static;

	/// Register a closure to be invoked for this event.
	#[cfg(feature = "threadsafe")]
	fn register<H>(&mut self, mut handler: H)
	where
		X: FnMut(&H, &A) + Send + 'static;

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
		X: FnMut(O, &A) -> F + 'static,
		F: Future<Output = ()> + 'static;

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
		X: FnMut(O, &A) -> F + Send + 'static,
		F: Future<Output = ()> + 'static;
}


impl<H, O, A, T> EventExt<H, O, A> for T
where
	T: Event<H, O, A>,
{
	#[cfg(not(feature = "threadsafe"))]
	fn register<X>(&mut self, mut handler: X)
	where
		X: FnMut(&H, &A) + 'static,
	{
		self.register_handler(EventHandler::Sync(Box::new(move |h, args| {
			handler(h, args);
		})));
	}

	#[cfg(feature = "threadsafe")]
	fn register<X>(&mut self, mut handler: X)
	where
		X: FnMut(&H, &A) + Send + 'static,
	{
		self.register_handler(EventHandler::Sync(Box::new(move |h, args| {
			handler(h, args);
		})));
	}

	#[cfg(not(feature = "threadsafe"))]
	fn register_async<X, F>(&mut self, mut handler: X)
	where
		X: FnMut(O, &A) -> F + 'static,
		F: Future<Output = ()> + 'static,
	{
		self.register_handler(EventHandler::Async(Box::new(move |h, args| {
			Box::pin(handler(h, args))
		})));
	}

	#[cfg(feature = "threadsafe")]
	fn register_async<X, F>(&mut self, mut handler: X)
	where
		X: FnMut(O, &A) -> F + Send + 'static,
		F: Future<Output = ()> + 'static,
	{
		self.register_handler(EventHandler::Async(Box::new(move |h, args| {
			Box::pin(handler(h, args))
		})));
	}
}


#[doc(hidden)]
#[macro_export]
macro_rules! decl_event {
	($name:ident < $owner:ty >) => {
		pub struct $name {
			pub(crate) owner: crate::rc::Weak<$owner>,
		}

		impl $name {
			#[allow(dead_code)]
			pub(crate) fn new(owner: crate::rc::Weak<$owner>) -> Self { Self { owner } }
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! decl_browser_event {
	($name:ident) => {
		decl_event!($name<BrowserWindowOwner>);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! def_event {
	( $name:ident<$handle_type:ty, $owner_type:ty, $arg_type:ty> (&mut $this:ident, $arg_name:ident) $body:block ) => {
		impl crate::event::Event<$handle_type, $owner_type, $arg_type> for $name {
			fn register_handler(&mut $this, $arg_name: crate::event::EventHandler<$handle_type, $owner_type, $arg_type>) $body
		}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! def_browser_event {
	( $name:ident<$arg_type:ty> (&mut $this:ident, $arg_name:ident) $body:block ) => {
		def_event!($name<BrowserWindowHandle, BrowserWindow, $arg_type> (&mut $this, $arg_name) $body);
	}
}
