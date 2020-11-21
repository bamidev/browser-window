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
/*BOOL CALLBACK _bw_Window_closeChild( HWND handle, LPARAM lparam );
// Returns whether or not the window still has children, that haven't been dropped.
bool _bw_Window_hasUndroppedChildren( const bw_Window* window );
BOOL CALLBACK _bw_Window_isDroppedCheck( HWND handle, LPARAM lparam );*/
LRESULT CALLBACK bw_Window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp);
void bw_WindowWin32_calculatePositionCentered( int width, int height, int* x, int* y );



size_t bw_Window_getTitle( bw_Window* window, bw_StrSlice title ) {

    // First get the length of the window title
    int length = GetWindowTextLengthW( window->impl.handle );
    BW_WIN32_ASSERT_SUCCESS;

    if ( length > 0 && title.len > 0 ) {
        WCHAR* buffer = (WCHAR*)malloc( sizeof(WCHAR) * (length + 1) );

        // Copy string
        int copied = GetWindowTextW( window->impl.handle, (LPSTR)buffer, length + 1 );
        if ( copied == 0 ) {
            free( buffer );
            BW_WIN32_PANIC_LAST_ERROR;
        }

        // Copy into our slice
        char* utf8_str;
        bw_win32_copyAsNewUtf8Str( buffer, &utf8_str );
        memcpy( title.data, utf8_str, length );
        free( utf8_str );
    }

    return length;
}

void bw_WindowImpl_destroy( bw_Window* window ) {
	DestroyWindow( window->impl.handle );
}

void bw_WindowImpl_hide( bw_Window* window ) {

	// Hide window and hide all its children, to emulate DestroyWindow without actually destroying it:
	ShowWindow( window->impl.handle, SW_HIDE );
}

bw_WindowImpl bw_WindowImpl_new(
	const bw_Window* window,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options
) {
	bw_WindowImpl impl;

	DWORD window_style = WS_OVERLAPPEDWINDOW;

	if ( !options->borders )
		window_style ^= WS_BORDER;
	if ( !options->resizable )
		window_style ^= WS_SIZEBOX & WS_MAXIMIZEBOX;
	if ( !options->minimizable )
		window_style ^= WS_MINIMIZEBOX;

	if ( width == -1 || height == -1 )
	    width = CW_USEDEFAULT;

	wchar_t* title = bw_win32_copyAsNewWstr( _title );

    // Set extended window style to 'layered', if opacity is set
	DWORD ex_style = 0;
	if ( options->opacity > 0 )
	    ex_style = WS_EX_LAYERED;


	// Create the window
	impl.handle = CreateWindowExW(
	    ex_style,
		L"bw-window",
		title,
		window_style,
		CW_USEDEFAULT,  // Let Windows decide where to place our window
		0,
		width,
		height,
		HWND_DESKTOP,	// Always set the window to be top level. Parent relationships are dealt with ourself
		NULL,
		window->app->impl.handle,
		(void*)window
	);
	free( title );
	if ( impl.handle == NULL ) {
		BW_WIN32_PANIC_LAST_ERROR;
	}

	// Store a pointer to our window handle in win32's window handle
	SetWindowLongPtrW( impl.handle, GWLP_USERDATA, (LONG_PTR)window );

	// Set opacity
	if ( options->opacity > 0 )
	    SetLayeredWindowAttributes( impl.handle, 0, 255 - options->opacity, LWA_ALPHA );

	// Show window
	ShowWindow( impl.handle, SW_SHOW );

	// Increase the application's window counter
	window->app->windows_alive += 1;

	return impl;
}

LRESULT CALLBACK bw_Window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
	bw_Window* window = (bw_Window*)GetWindowLongPtrW( hwnd, GWLP_USERDATA );

	switch (msg) {
	case WM_SIZE:
		BW_ASSERT( window != 0, "Invalid window pointer during WM_SIZE" );

		RECT rect;
		GetClientRect( window->impl.handle, &rect );

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
		break;
	default:
		return DefWindowProcW(hwnd, msg, wp, lp);
	}

	return 0;
}

void bw_Window_setTitle( bw_Window* window, bw_CStrSlice _title ) {
    WCHAR* title = bw_win32_copyAsNewWstr( _title );

    SetWindowTextW( window->impl.handle, title );

    free( title );
}

void bw_WindowImpl_show( bw_Window* window ) {
	ShowWindow( window->impl.handle, SW_SHOW );
}

void bw_WindowWin32_calculatePositionCentered( int width, int height, int* x, int* y ) {
    RECT rect;

    GetClientRect( GetDesktopWindow(), &rect );

    int desktop_width = rect.right - rect.left;
    int desktop_height = rect.bottom - rect.top;

    *x = ( desktop_width - width ) / 2;
    *y = ( desktop_height - height ) / 2;
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
