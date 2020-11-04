#include "win32.h"
#include "../application/win32.h"
#include "../assert.h"
#include "../debug.h"
#include "../win32.h"
#include "../window.h"

#include <stdio.h>
#include <stdlib.h>



typedef struct {
	const bw_Window* window;
	bool all_dropped;
} bw_Window_DropCheckData;

// The callback that is called when enumerating over child windows.
BOOL CALLBACK _bw_Window_closeChild( HWND handle, LPARAM lparam );
// Frees the window and cleans up everything thats part of it.
void _bw_Window_free( bw_Window* window );
// Returns whether or not the window still has children, that haven't been dropped.
bool _bw_Window_hasUndroppedChildren( const bw_Window* window );
BOOL CALLBACK _bw_Window_isDroppedCheck( HWND handle, LPARAM lparam );
LRESULT CALLBACK _bw_Window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp);



void bw_Window_close( bw_Window* window ) {

	if ( window->callbacks.on_close != 0 )
		window->callbacks.on_close( window );

	// Hide window and hide all its children, to emulate DestroyWindow without actually destroying it:
	ShowWindow( window->handle, SW_HIDE );

	EnumWindows( _bw_Window_closeChild, (LPARAM)window );

	window->closed = true;
}

void bw_Window_drop( bw_Window* window ) {
	window->dropped = true;

	// Only destroy this window if it is already closed.
	// If it is not already closed, it will get destroyed the moment is gets closed.
	if ( window->closed ) {
		DestroyWindow( window->handle );
	}
}

bw_Window* bw_Window_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options,
	void* user_data
) {
	bw_Window* window = (bw_Window*)malloc( sizeof( bw_Window ) );
	window->app = app;
	window->parent = parent;
	window->closed = false;
	window->dropped = false;
	window->user_data = user_data;
	memset( &window->callbacks, 0, sizeof( bw_WindowCallbacks ) );


	DWORD window_style = WS_OVERLAPPEDWINDOW;

	if ( !options->borders )
		window_style ^= WS_BORDER;
	if ( !options->resizable )
		window_style ^= WS_SIZEBOX;
	if ( !options->minimizable )
		window_style ^= WS_MINIMIZEBOX;
	if ( !options->maximizable )
		window_style ^= WS_MAXIMIZEBOX;

	wchar_t* title = bw_win32_copyAsNewWstr( _title );

	// Create the window
	window->handle = CreateWindowExW( 0,
		L"bw-window",
		title,
		window_style,
		0, 0,
		width,
		height,
		HWND_DESKTOP,	// Always set the window to be top level. Parent relationships are dealt with ourself
		NULL,
		app->handle,
		(void*)window
	);
	free( title );
	if ( window->handle == NULL ) {
		BW_WIN32_ASSERT_ERROR;
	}

	// Store a pointer to our window handle in win32's window handle
	SetWindowLongPtr( window->handle, GWLP_USERDATA, (LONG_PTR)window );

	// Show window
	ShowWindow( window->handle, SW_SHOW );

	// Increase the application's window counter
	app->windows_alive += 1;

	return window;
}

LRESULT CALLBACK bw_Window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
	bw_Window* window = (bw_Window*)GetWindowLongPtrW( hwnd, GWLP_USERDATA );

	switch (msg) {
	case WM_SIZE:
		BW_ASSERT( window != 0, "Invalid window pointer during WM_SIZE" );

		RECT rect;
		GetClientRect( window->handle, &rect );

		if ( window->callbacks.on_resize != 0 ) {

			unsigned int width = rect.right - rect.left;
			unsigned int height = rect.bottom - rect.top;

			window->callbacks.on_resize( window, width, height );
		}
		break;
	/*case WM_DPICHANGED: {
		auto rect = reinterpret_cast<LPRECT>(lp);
		auto x = rect->left;
		auto y = rect->top;
		auto w = rect->right - x;
		auto h = rect->bottom - y;
		SetWindowPos(hwnd, nullptr, x, y, w, h, SWP_NOZORDER | SWP_NOACTIVATE);
		break;
	}*/
	// When closing the window, only destroy it when it is ready for it to be destroyed
	case WM_CLOSE:
		bw_Window_close( window );

		// If the window handle has been dropped, also destroy the window
		if ( window->dropped )
			DestroyWindow( hwnd );
		break;
	case WM_DESTROY:
		_bw_Window_free( window );
		break;
	default:
		return DefWindowProcW(hwnd, msg, wp, lp);
	}

	return 0;
}

BOOL CALLBACK _bw_Window_closeChild( HWND handle, LPARAM _window ) {
	bw_Window* window = (bw_Window*)_window;

	bw_Window* enum_window = (bw_Window*)GetWindowLongPtrW( handle, GWLP_USERDATA );

	// Check if user data was set for the win32 handle
	if ( enum_window != 0 ) {

		// Read the class name for the win32 handle
		wchar_t class_name[11];
		int class_name_len = GetClassNameW( handle, class_name, 11 );

		// Check if this handle is of class bw-window,
		//  if it is, we know that the enum_window handle is valid.
		wchar_t* bw_class = L"bw-window";
		if ( class_name_len == 9 && memcmp( class_name, bw_class, 9*sizeof(wchar_t) ) == 0 ) {

			// If this is a child window, close it
			if ( enum_window->parent == window ) {
				bw_Window_close( enum_window );
			}
		}

	}

	return true;
}

void _bw_Window_free( bw_Window* window ) {

	// The user data has been allocated outside of the c context, so we can't free it because it might not be allocated with C's malloc or the like.
	if ( window->callbacks.do_cleanup != 0 )
		window->callbacks.do_cleanup( window );

	// Check if we need to destroy & free our parent window as well.
	// It might not have been destroyed because this window has still not dropped.
	if ( window->parent != 0 && window->parent->closed ) {
		DestroyWindow( window->parent->handle );
		// DestroyWindow will also invoke _bw_Window_free for the parent window.
	}

	// Finally, free the memory we allocated to store the handle and everything related to the window
	free( window );

	// Decrease the window counter
	bw_Application* app = window->app;
	app->windows_alive -= 1;

	// Exit application if this was our last window
	if ( app->windows_alive == 0 )
		bw_Application_exit( app, 0 );
}
