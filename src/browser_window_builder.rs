use browser_window_ffi::*;

use std::marker::PhantomData;

use crate::application::{Application, ApplicationAsync, ApplicationHandle};
use crate::browser_window::{BrowserWindow, BrowserWindowHandle, BrowserWindowInner, BrowserWindowAsync};

use std::{
	mem,
	ops::Deref,
	ptr,
	sync::Arc
};



pub enum Source {
	Url( String ),
	Html( String )
}

pub struct BrowserWindowBuilder {

	parent: Option<BrowserWindowHandle>,
	source: Source,
	title: Option<String>,
	width: Option<u32>,
	height: Option<u32>,
	handler: Option<Box<dyn FnMut(BrowserWindowHandle, &str, &[&str]) + Send>>
}



impl BrowserWindowBuilder {
	pub fn handler<H>( mut self, handler: H ) -> Self where
		H: FnMut(BrowserWindowHandle, &str, &[&str]) + Send + 'static
	{
		self.handler = Some( Box::new( handler ) );
		self
	}

	pub fn parent<B>( mut self, bw: &B ) -> Self where
		B: Deref<Target=BrowserWindowHandle>
	{
		self.parent = Some( (**bw).clone() );
		self
	}

	pub fn new( source: Source ) -> Self {
		Self {
			parent: None,
			source: source,
			handler: None,
			title: None,
			width: None,
			height: None
		}
	}

	pub fn title( mut self, title: String ) -> Self {
		self.title = Some( title );
		self
	}

	pub fn width( mut self, width: u32 ) -> Self {
		self.width = Some( width );
		self
	}

	pub fn height( mut self, height: u32 ) -> Self {
		self.height = Some( height );
		self
	}

	pub fn spawn( self, app: &Application ) -> BrowserWindow {
		BrowserWindow {
			inner: self._spawn( (*app.inner).clone() ),
			_not_send: PhantomData
		}
	}

	fn _spawn( self, app: ApplicationHandle ) -> Arc<BrowserWindowInner> {

		match self {
			BrowserWindowBuilder { parent, source, title, width, height, handler } => {

				// Parent
				let parent_handle = match parent {
					None => ptr::null(),
					Some( p ) => p._ffi_handle
				};

				// Source
				let csource = match &source {	// Use a reference, we want source to live until the end of the function because bw_BrowserWindowSource holds a reference to its internal string.
					Source::Url( url ) => { bw_BrowserWindowSource {
						data: url.as_str().into(),
						is_html: false
					} },
					Source::Html( html ) => { bw_BrowserWindowSource {
						data: html.as_str().into(),
						is_html: true
					} }
				};

				let title_ptr = match title.as_ref() {
					None => "Browser Window".into(),
					Some( t ) => t.as_str().into()
				};

				// Width and height use -1 as the 'use default' option within the c code
				let w: i32 = match width {
					Some(w) => w as i32,
					None => -1
				};
				let h: i32 = match height {
					Some(h) => h as i32,
					None => -1
				};

				let user_data = Box::into_raw( Box::new(
					BrowserWindowHandlerData {
						func: match handler {
							None => Box::new(|_,_,_|{}),
							Some(f) => Box::new(f)
						}
					}
				) );

				let window_options = bw_WindowOptions {
					maximizable: true,
					minimizable: true,
					resizable: true,
					closable: true,
					borders: true,
					is_popup: true
				};
				let other_options = bw_BrowserWindowOptions {
					dev_tools: true
				};

				let ffi_handle = unsafe { bw_BrowserWindow_new(
					app._ffi_handle.clone(),
					parent_handle,
					csource,
					title_ptr,
					w,
					h,
					&window_options,
					&other_options,
					ffi_window_invoke_handler,
					user_data as _
				) };

				Arc::new( BrowserWindowInner {
					app: app,
					handle: BrowserWindowHandle { _ffi_handle: ffi_handle }
				} )
			}
		}
	}

	pub async fn spawn_async( self, app: &ApplicationAsync ) -> BrowserWindowAsync {

		let app_inner: ApplicationHandle = (*app.inner).clone();
		
		let bw_inner = *app.dispatch(move |_| {
			self._spawn( app_inner )
		}).await;

		BrowserWindowAsync {
			inner: bw_inner
		}
	}
}

struct BrowserWindowHandlerData {
	func: Box<dyn FnMut( BrowserWindowHandle, &str, &[&str])>
}



fn args_to_vec<'a>( args: *const bw_CStrSlice, args_count: usize ) -> Vec<&'a str> {

	let mut vec = Vec::<&str>::with_capacity( args_count );

	for i in 0..args_count {
		let str_ref: &str = unsafe { (*args.offset(i as _)).into() };

		vec.push( str_ref );
	}

	vec
}

extern "C" fn ffi_window_invoke_handler( inner_handle: *mut bw_BrowserWindow, _command: bw_CStrSlice, _args: *const bw_CStrSlice, args_count: usize ) {

	unsafe {
		let data_ptr: *mut BrowserWindowHandlerData = mem::transmute( bw_BrowserWindow_get_user_data( inner_handle ) );
		let data: &mut BrowserWindowHandlerData = &mut *data_ptr;

		match data {
			BrowserWindowHandlerData{ func } => {
				let outer_handle = BrowserWindowHandle::from_ptr( inner_handle );

				let args = args_to_vec( _args, args_count );

				func( outer_handle, _command.into(), &*args );
			}
		}
	}
}
