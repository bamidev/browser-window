pub mod c;

pub use c::WindowImpl;

use crate::prelude::*;



pub trait WindowExt: Copy + Default {
	fn app( &self ) -> ApplicationImpl;

	fn destroy( &self );
	fn drop( &self );

	fn get_content_dimensions( &self ) -> Dims2D;
	fn get_opacity( &self ) -> u8;
	fn get_position( &self ) -> Pos2D;
	fn get_title( &self ) -> String;
	fn get_window_dimensions( &self ) -> Dims2D;

	fn hide( &self );

	fn set_content_dimensions( &self, dimensions: Dims2D );
	fn set_opacity( &self, opacity: u8 );
	fn set_position( &self, position: Pos2D );
	fn set_title( &self, title: &str );
	fn set_window_dimensions( &self, dimensions: Dims2D );

	fn show( &self );
}

pub type WindowOptions = cbw_WindowOptions;