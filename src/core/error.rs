#[cfg(not(feature = "webkit"))]
mod c;
#[cfg(feature = "webkit")]
mod common;

#[cfg(not(feature = "webkit"))]
pub use c::Error;
#[cfg(feature = "webkit")]
pub use common::Error;

pub type Result<T> = std::result::Result<T, Error>;
