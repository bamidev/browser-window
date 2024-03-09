//! Some common traits that need to be often available.

#[cfg(feature = "threadsafe")]
pub use super::delegate::{DelegateError, DelegateFuture, DelegateFutureFuture};
pub use crate::{core::prelude::*, event::EventExt, javascript::JsValue};
