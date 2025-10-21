//! This module contains all event related types.
//!
//! To understand how events work in _BrowserWindow_, here is a short summary.
//! The [`BrowserWindow`](../browser/struct.BrowserWindow.html) handle
//! contains a bunch of functions that return different event objects, and then
//! the event object can be used to register the callback at.
//!
//! Each event object has the
//! [`register`](trait.EventExt.html#tymethod.register) method to register a
//! closure to be executed on the occurence of the event.
//!
//! ```
//! use browser_window::{browser::*, prelude::*};
//!
//! fn example(bw: BrowserWindow) {
//! 	bw.on_message()
//! 		.register(|h: &BrowserWindowHandle, e: MessageEventArgs| {
//! 			// .. code here ...
//! 		});
//! }
//! ```
//!
//! There is also a
//! [`register_async`](trait.EventExt.html#tymethod.register_async)
//! method, which can be useful in async code:
//!
//! ```
//! use browser_window::{browser::*, prelude::*};
//!
//! fn async_example(bw: BrowserWindow) {
//! 	bw.on_message()
//! 		.register_async(|h: BrowserWindow, e: MessageEventArgs| async move {
//! 			// .. code here ...
//! 		});
//! }
//! ```
//!
//! Also, keep in mind that if the `register` or `register_async` methods are
//! not available, that means that event object does not implement `EventExt`,
//! which in turn means that the corresponding event is not supported for the
//! browser framework that has been selected.
//!
//! CEF supports all events, unless it is stated that it is not implemented yet.
//! The other browser frameworks only support a subset of what is supported for
//! CEF. The reason for this is that CEF is simply the most cross-platform
//! framework out there, so it gets the most care.

use std::{boxed::Box, future::Future, pin::Pin};

#[cfg(not(feature = "threadsafe"))]
pub type EventHandlerAsyncCallback<O, A> =
	dyn FnMut(O, A) -> Pin<Box<dyn Future<Output = ()> + 'static>> + 'static;
#[cfg(not(feature = "threadsafe"))]
pub type EventHandlerSyncCallback<H, A> = dyn FnMut(&H, A) + 'static;
#[cfg(feature = "threadsafe")]
pub type EventHandlerAsyncCallback<O, A> =
	dyn FnMut(O, A) -> Pin<Box<dyn Future<Output = ()> + 'static>> + Send + 'static;
#[cfg(feature = "threadsafe")]
pub type EventHandlerSyncCallback<H, A> = dyn FnMut(&H, A) + Send + 'static;

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
		X: FnMut(&H, A) + 'static;

	/// Register a closure to be invoked for this event.
	#[cfg(feature = "threadsafe")]
	fn register<X>(&mut self, handler: X)
	where
		X: FnMut(&H, A) + Send + 'static;

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
		X: FnMut(O, A) -> F + 'static,
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
	fn register_async<X, F>(&mut self, handler: X)
	where
		X: FnMut(O, A) -> F + Send + 'static,
		F: Future<Output = ()> + 'static;
}

impl<H, O, A, T> EventExt<H, O, A> for T
where
	T: Event<H, O, A>,
{
	#[cfg(not(feature = "threadsafe"))]
	fn register<X>(&mut self, mut handler: X)
	where
		X: FnMut(&H, A) + 'static,
	{
		self.register_handler(EventHandler::Sync(Box::new(move |h, args| {
			handler(h, args);
		})));
	}

	#[cfg(feature = "threadsafe")]
	fn register<X>(&mut self, mut handler: X)
	where
		X: FnMut(&H, A) + Send + 'static,
	{
		self.register_handler(EventHandler::Sync(Box::new(move |h, args| {
			handler(h, args);
		})));
	}

	#[cfg(not(feature = "threadsafe"))]
	fn register_async<X, F>(&mut self, mut handler: X)
	where
		X: FnMut(O, A) -> F + 'static,
		F: Future<Output = ()> + 'static,
	{
		self.register_handler(EventHandler::Async(Box::new(move |h, args| {
			Box::pin(handler(h, args))
		})));
	}

	#[cfg(feature = "threadsafe")]
	fn register_async<X, F>(&mut self, mut handler: X)
	where
		X: FnMut(O, A) -> F + Send + 'static,
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
			#[allow(dead_code)]
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
