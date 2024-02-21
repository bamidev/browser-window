pub use browser_window_c::*;

pub use super::cookie::*;

pub use core::prelude::*;



pub struct Dims2D(pub(crate) cbw_Dims2D);
pub struct Pos2D(pub(crate) cbw_Pos2D);



pub use super::application::{ApplicationExt, ApplicationImpl};
pub use super::browser_window::{BrowserWindowExt, BrowserWindowImpl};
pub use super::window::{WindowExt, WindowImpl};



impl Dims2D {
	pub fn new(width: u16, height: u16) -> Dims2D {
		Dims2D(cbw_Dims2D { width: width, height: height })
	}

	pub fn width(&self) -> u16	{ self.0.width }
	pub fn height(&self) -> u16	{ self.0.height }
}

impl Pos2D {
	pub fn new(x: u16, y: u16) -> Pos2D {
		Pos2D(cbw_Pos2D {x: x, y: y})
	}

	pub fn x(&self) -> u16	{ self.0.x }
	pub fn y(&self) -> u16	{ self.0.y }
}