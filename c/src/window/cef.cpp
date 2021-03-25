#include "../window.h"

#include "../cef/util.hpp"
#include "../common.h"

#include <include/cef_base.h>
#include <include/views/cef_window.h>



class MyWindowDelegate : public CefWindowDelegate {
	bw_WindowOptions options;

public:
	MyWindowDelegate( const bw_WindowOptions& options ) : options(options) {}

	bool CanClose( CefRefPtr<CefWindow> window ) override {
		UNUSED( window );
		return true;
	}

	bool CanMaximize( CefRefPtr<CefWindow> window ) override {
		UNUSED( window );
		return true;
	}

	bool CanMinimize( CefRefPtr<CefWindow> window ) override {
		UNUSED( window );
		return this->options.minimizable;
	}

	bool CanResize( CefRefPtr<CefWindow> window ) override {
		UNUSED( window );
		return this->options.resizable;
	}

	CefRefPtr<CefWindow> GetParentWindow( CefRefPtr<CefWindow> window, bool* is_menu, bool* can_activate_menu ) override {
		UNUSED( window );
		UNUSED( is_menu );
		UNUSED( can_activate_menu );
		return NULL;
	}

	bool IsFrameless( CefRefPtr<CefWindow> window ) override {
		UNUSED( window );
		return !this->options.borders;
	}

	bool OnAccelerator( CefRefPtr<CefWindow> window, int command_id ) override {
		UNUSED( window );
		UNUSED( command_id );
		return false;
	}

	bool OnKeyEvent( CefRefPtr<CefWindow> window, const CefKeyEvent& event ) override {
		UNUSED( window );
		return false;
	}

	void OnWindowCreated( CefRefPtr<CefWindow> window ) override {
		UNUSED( window );
	}

	void OnWindowDestroyed( CefRefPtr<CefWindow> window ) override {
		UNUSED( window );
	}

protected:
	IMPLEMENT_REFCOUNTING(MyWindowDelegate);
};



// Opacity is not supported with CEF's window API.
uint8_t bw_Window_getOpacity( bw_Window* window ) {
	return 255;
}

void bw_Window_setOpacity( bw_Window* window, uint8_t opacity ) {
	UNUSED( window );
	UNUSED( opacity );
}

bw_WindowImpl bw_WindowImpl_new(
	const bw_Window* _window,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options
) {
	UNUSED( _window );

	CefRefPtr<CefWindowDelegate> cef_window_options( new MyWindowDelegate( *options ) );
	CefRefPtr<CefWindow> window = CefWindow::CreateTopLevelWindow( cef_window_options );

	window->SetTitle( bw_cef_copyToString( _title ) );

	CefSize size( width, height );
	window->SetSize( size );

	bw_WindowImpl impl;
	impl.handle_ptr = new CefRefPtr<CefWindow>( window );
	return impl;
}

void bw_WindowImpl_destroy( bw_WindowImpl* window ) {
	delete (CefRefPtr<CefWindow>*)window->handle_ptr;
}

void bw_WindowImpl_hide( bw_WindowImpl* window ) {
	(*(CefRefPtr<CefWindow>*)window->handle_ptr)->Hide();
}

void bw_WindowImpl_show( bw_WindowImpl* window ) {
	(*(CefRefPtr<CefWindow>*)window->handle_ptr)->Show();
}