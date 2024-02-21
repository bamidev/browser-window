//! Error types.

pub use crate::core::error::{CbwError, CbwResult};

use std::fmt;



#[derive(Debug)]
pub enum Error {
	Cbw(CbwError)
}

pub type Result<T> = std::result::Result<T, Error>;



impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Cbw(e) => write!(f, "c(bw) error: {}", e)
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}

impl From<CbwError> for Error {
	fn from(e: CbwError) -> Self {
		Self::Cbw(e)
	}
}