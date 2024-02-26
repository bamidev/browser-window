#[cfg(not(feature = "webkitgtk"))]
mod c;
#[cfg(feature = "webkitgtk")]
mod common;

#[cfg(not(feature = "webkitgtk"))]
pub use c::Error;
#[cfg(feature = "webkitgtk")]
pub use common::Error;

pub type Result<T> = std::result::Result<T, Error>;
