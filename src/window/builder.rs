use super::*;

use unsafe_send_sync::UnsafeSend;



pub struct WindowBuilder {
    pub(in crate) borders: bool,
    pub(in crate) height: i32,
    pub(in crate) minimizable: bool,
    pub(in crate) opacity: u8,
    pub(in crate) parent: Option<UnsafeSend<WindowHandle>>,
    pub(in crate) resizable: bool,
    pub(in crate) title: Option<String>,
    pub(in crate) width: i32
}



impl WindowBuilder {

    /// Sets whether or not the window has borders.
    /// Default is true.
    pub fn borders( &mut self, value: bool ) -> &mut Self {
        self.borders = value;	self
    }

    /// Sets the height that the browser window will be created with initially
    pub fn height( &mut self, height: u32 ) -> &mut Self {
        self.height = height as i32;
        self
    }

    /// Sets whether or not the window has a minimize button on the title bar
    /// Default is true
    pub fn minimizable( &mut self, value: bool ) -> &mut Self {
        self.minimizable = value;	self
    }

    /// Makes the window transparent.
    /// A `value` of `255` indicates that the window should be fully transparent.
    /// A `value` of `0` indicates the the window should not be transparent at all.
    pub fn opacity( &mut self, value: u8 ) -> &mut Self {
        self.opacity = value;	self
    }

    /// Configure a parent window.
    /// When a parent window closes, this browser window will close as well.
    /// This could be a reference to a `Browser` or `BrowserThreaded` handle.
    pub fn parent<W>( &mut self, bw: &W ) -> &mut Self where
        W: OwnedWindow
    {
        self.parent = Some( UnsafeSend::new( bw.window_handle() ) );
        self
    }

    pub fn new() -> Self {
        Self {
            borders: true,
            height: -1,
            minimizable: true,
            opacity: 0,
            parent: None,
            resizable: true,
            title: None,
            width: -1
        }
    }

    /// Sets the width and height of the browser window
    pub fn size( &mut self, width: u32, height: u32 ) -> &mut Self {
        self.width = width as i32;
        self.height = height as i32;
        self
    }

    /// Sets the title of the window.
    pub fn title<S: Into<String>>( &mut self, title: S ) -> &mut Self {
        self.title = Some( title.into() );
        self
    }


    /// Sets the width that the browser window will be created with initially.
    pub fn width( &mut self, width: u32 ) -> &mut Self {
        self.width = width as i32;
        self
    }

    /// Sets whether or not the window will be resizable.
    /// Default is true.
    pub fn resizable( &mut self, resizable: bool ) -> &mut Self {
        self.resizable = resizable;	self
    }
}