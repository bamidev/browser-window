#include "win32.h"
#include "../application/win32.h"
#include "../assert.h"
#include "../debug.h"
#include "../win32.h"
#include "../window.h"

#include <stdio.h>
#include <stdlib.h>

#include <windef.h>
#include <windows.h>



typedef struct {
	const bw_Window* window;
	bool all_dropped;
} bw_Window_DropCheckData;

// The callback that is called when enumerating over child windows.
/*BOOL CALLBACK _bw_Window_closeChild( HWND handle, LPARAM lparam );
// Returns whether or not the window still has children, that haven't been dropped.
bool _bw_Window_hasUndroppedChildren( const bw_Window* window );
BOOL CALLBACK _bw_Window_isDroppedCheck( HWND handle, LPARAM lparam );*/
void bw_WindowWin32_calculatePositionCentered( int width, int height, int* x, int* y );



bw_Dims2D bw_Window_getContentDimensions( bw_Window* window ) {
	bw_Dims2D dims;

	RECT rect;
	GetClientRect( window->impl.handle, &rect );

	dims.width = (uint16_t)(rect.right - rect.left);
	dims.height = (uint16_t)(rect.bottom - rect.top);

	return dims;
}

uint8_t bw_Window_getOpacity( bw_Window* window ) {
	// For some reason the GetLayeredWindowAttributes function gives an "Invalid access to memory loaction [998]" error when trying to access the LWA_ALPHA property.
	// This is a workaround.
	return window->impl.opacity;
}

bw_Pos2D bw_Window_getPosition( bw_Window* window ) {
	bw_Pos2D pos;

	RECT rect;
	GetWindowRect( window->impl.handle, &rect );

	pos.x = (uint16_t)rect.left;
	pos.y = (uint16_t)rect.top;

	return pos;
}

size_t bw_Window_getTitle( bw_Window* window, char** title ) {

	// First get the length of the window title
	int length = GetWindowTextLengthW( window->impl.handle );
	BW_WIN32_ASSERT_SUCCESS;

	if ( length > 0 ) {
		WCHAR* buffer = (WCHAR*)malloc( sizeof(WCHAR) * (length + 1) );

		// Copy string
		int copied = GetWindowTextW( window->impl.handle, (LPWSTR)buffer, length + 1 );
		if ( copied == 0 ) {
			free( buffer );
			BW_WIN32_PANIC_LAST_ERROR;
		}

		size_t l = bw_win32_copyAsNewUtf8Str( buffer, title );
		BW_ASSERT(length == l, "UTF-8 string length is invalid")
	}

	return length;
}

bw_Dims2D bw_Window_getWindowDimensions( bw_Window* window ) {
	bw_Dims2D dims;

	RECT rect;
	GetWindowRect( window->impl.handle, &rect );

   dims.width = (uint16_t)(rect.right - rect.left);
   dims.height = (uint16_t)(rect.bottom - rect.top);

   return dims;
}

void bw_Window_setContentDimensions( bw_Window* window, bw_Dims2D dimensions ) {
	RECT rect;
	rect.left = 0; rect.right = dimensions.width;
	rect.top = 0;  rect.bottom = dimensions.height;

	// Obtained the window size based on our client area size
	if ( !AdjustWindowRect( &rect, window->impl.style, FALSE ) )
		BW_WIN32_PANIC_LAST_ERROR

	LONG actual_width = rect.right - rect.left;
	LONG actual_height = rect.bottom - rect.top;

	// Apply the current position with the new width and height
	if ( !SetWindowPos( window->impl.handle, 0, rect.left, rect.top, actual_width, actual_height, SWP_NOMOVE  ) )
		BW_WIN32_PANIC_LAST_ERROR
}

void bw_Window_setOpacity( bw_Window* window, uint8_t opacity ) {
	
	if ( !SetLayeredWindowAttributes( window->impl.handle, 0, opacity, LWA_ALPHA ) )
		BW_WIN32_PANIC_LAST_ERROR
	window->impl.opacity = opacity;
}

void bw_Window_setPosition( bw_Window* window, bw_Pos2D position ) {

	if ( !SetWindowPos( window->impl.handle, 0, position.x, position.y, 0, 0, SWP_NOSIZE ) )
		BW_WIN32_PANIC_LAST_ERROR
}

void bw_Window_setTitle( bw_Window* window, bw_CStrSlice _title ) {
	WCHAR* title = bw_win32_copyAsNewWstr( _title );

	SetWindowTextW( window->impl.handle, title );

	free( title );
}

void bw_Window_setWindowDimensions( bw_Window* window, bw_Dims2D dimensions ) {

	if ( !SetWindowPos( window->impl.handle, 0, 0, 0, dimensions.width, dimensions.height, SWP_NOMOVE ) )
		BW_WIN32_PANIC_LAST_ERROR
}



void bw_WindowImpl_close(bw_WindowImpl* window) {
	// Hides the window so that it is perceived as 'closed'.
	if (window->closed = FALSE) {
		ShowWindow(window->handle, SW_HIDE);
		window->closed = TRUE;
	}
}

void bw_WindowImpl_hide( bw_WindowImpl* window ) {

	// // Hide window and hide all its children, to emulate DestroyWindow without actually destroying it:
	ShowWindow( window->handle, SW_HIDE );
}

void* bw_WindowImpl_innerHandle(bw_WindowImpl* window) {
	return window->handle;
}

bw_WindowImpl bw_WindowImpl_new(
	const bw_Window* window,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options
) {
	bw_WindowImpl impl;

	impl.style = options->decorated ? WS_OVERLAPPEDWINDOW : 0;

	if ( !options->borders )
		impl.style ^= WS_BORDER;
	if ( !options->resizable )
		impl.style ^= WS_SIZEBOX & WS_MAXIMIZEBOX;
	if ( !options->minimizable )
		impl.style ^= WS_MINIMIZEBOX;

	if ( width == -1 && height == -1 )
		width = CW_USEDEFAULT;

	wchar_t* title = bw_win32_copyAsNewWstr( _title );

	// Create the window
	impl.handle = CreateWindowExW(
		WS_EX_LAYERED,
		L"bw-window",
		title,
		impl.style,
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
	BW_WIN32_ASSERT_SUCCESS;

	// We give the window an ex-style of WS_EX_LAYERED.
	// This means however that we need to explicitly set the opacity to a value.
	// We default to 255 for no transparency.
	impl.opacity = 255;
	if ( !SetLayeredWindowAttributes( impl.handle, 0, impl.opacity, LWA_ALPHA ) )
		BW_WIN32_PANIC_LAST_ERROR

	return impl;
}

LRESULT CALLBACK bw_Window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
	bw_Window* window = (bw_Window*)GetWindowLongPtrW( hwnd, GWLP_USERDATA );

	switch (msg) {
		case WM_SIZE: {
			BW_ASSERT( window != 0, "Invalid window pointer during WM_SIZE" );

			RECT rect;
			GetClientRect( window->impl.handle, &rect );
			bw_WindowWin32_onResize(window, rect.left, rect.right, rect.top, rect.bottom);
			break;
		}
		// Triggered when a user closes the window
		case WM_CLOSE: {
			if (window->user_data != NULL) {
				bw_Window_freeUserData(window);
				window->user_data = NULL;
			}
			ShowWindow(hwnd, SW_HIDE);
			break;
		}
		// Triggered when DestroyWindow is called, which is the only way for us to programmatically close the window.
		// So bw_Window_freeUserData may have been called multiple times, but it should have been implemented to account for that.
		case WM_DESTROY: {
			if (window->user_data != NULL) {
				bw_Window_freeUserData(window);
				window->user_data = NULL;
			}
			break;
		}
		default: {
			return DefWindowProcW(hwnd, msg, wp, lp);
		}
	}

	return 0;
}

void bw_WindowImpl_show( bw_WindowImpl* window ) {
	ShowWindow( window->handle, SW_SHOW );
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
	}

	return true;
}
