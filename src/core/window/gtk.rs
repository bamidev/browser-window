use std::sync::{Arc, atomic::AtomicI32};

use glib::object::ObjectExt;
use gtk::prelude::{GtkWindowExt, WidgetExt};

use super::{WindowExt, WindowOptions};
use crate::{core::application::ApplicationImpl, prelude::*};

#[derive(Clone)]
pub struct WindowImpl(pub gtk::Window);

impl WindowImpl {
	pub fn new(
		app: ApplicationImpl, parent: Self, title: &str, width: Option<u32>, height: Option<u32>,
		options: &WindowOptions,
	) -> Self {
		let mut builder = gtk::Window::builder()
			.application(&app.inner)
			.parent(&parent.0)
			.destroy_with_parent(true)
			.decorated(true)
			.title(title);

		builder = builder
			.border_width(options.borders as _)
			.resizable(options.resizable);

		if let Some(w) = width {
			builder = builder.width_request(w as _);
		}
		if let Some(h) = height {
			builder = builder.height_request(h as _);
		}

		let inner = builder.build();
		inner.set_keep_above(options.keep_above);

		// Delete user data when closing the window
		inner.connect_destroy(|this| {
			let user_data = unsafe { *this.data::<*mut ()>("bw-data").unwrap().as_ref() };
			BrowserWindowImpl::free_user_data(user_data);
		});

		Self(inner)
	}

	pub fn gtk_handle(&self) -> &gtk::Window { &self.0 }
}

impl Default for WindowImpl {
	fn default() -> Self { Self(gtk::Window::new(gtk::WindowType::Toplevel)) }
}

impl WindowExt for WindowImpl {
	fn app(&self) -> ApplicationImpl {
		ApplicationImpl {
			inner: self.0.application().unwrap(),
			exit_code: Arc::new(AtomicI32::new(0)),
		}
	}

	fn close(&self) { self.0.close(); }

	fn free(&self) {}

	fn content_dimensions(&self) -> Dims2D {
		unimplemented!();
	}

	fn opacity(&self) -> u8 { 0 }

	fn position(&self) -> Pos2D {
		let (x, y) = self.0.position();
		Pos2D {
			x: x as _,
			y: y as _,
		}
	}

	fn title(&self) -> String {
		self.0
			.title()
			.map(|g| g.to_string())
			.unwrap_or(String::new())
	}

	fn window_dimensions(&self) -> Dims2D {
		let (w, h) = self.0.size();
		Dims2D {
			width: w as _,
			height: h as _,
		}
	}

	fn hide(&self) { self.0.hide(); }

	fn set_content_dimensions(&self, _dimensions: Dims2D) {}

	fn set_opacity(&self, _opacity: u8) {}

	fn set_position(&self, _position: Pos2D) {
		unimplemented!();
	}

	fn set_title(&self, title: &str) { self.0.set_title(title); }

	fn set_user_data(&self, user_data: *mut ()) {
		unsafe {
			self.0.set_data("bw-data", user_data);
		}
	}

	fn set_window_dimensions(&self, dimensions: Dims2D) {
		self.0
			.set_size_request(dimensions.width as _, dimensions.height as _);
	}

	fn show(&self) { self.0.show_all(); }
}
