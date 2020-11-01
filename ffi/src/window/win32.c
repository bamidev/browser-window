#include "win32.h"
#include "../application/win32.h"
#include "../debug.h"
#include "../win32.h"
#include "../window.h"


#include <stdio.h>
#include <stdlib.h>



void bw_Window_cleanup( bw_Window* window );
void _bw_Window_destroy( bw_Window* window, void* data );
BOOL CALLBACK bw_Window_hideChild( HWND handle, LPARAM lparam );
void _bw_Window_resize( bw_Window* window );



void bw_Window_free( bw_Window* window ) {

	// The user data has been allocated outside of the c context, so we can't free it because it might not be allocated with C's malloc or the like.
	window->callbacks.do_cleanup( window );

	// Decrease the window counter
	bw_Application* app = window->app;
	app->windows_alive -= 1;

	// Finally, free the memory we allocated to store the handle and everything related to the window
	free( window );

	// Exit is this was our last window
	if ( app->windows_alive == 0 )
		bw_Application_exit( app, 0 );
}

void bw_Window_close( bw_Window* window ) {

	if ( window->callbacks.on_close != 0 )
		window->callbacks.on_close( window );

	// Hide window and hide all its children, to emulate DestroyWindow without actually destroying it:
	ShowWindow( window->handle, SW_HIDE );

	EnumChildWindows( window->handle, bw_Window_hideChild, 0 );

	window->closed = true;
}

void bw_Window_drop( bw_Window* window ) {

	// We need to dispatch the destruction of this window because bw_Window_drop needs to be thread-safe.
	bw_Window_dispatch( window, _bw_Window_destroy, 0 );
}

void _bw_Window_destroy( bw_Window* window, void* data ) {

	// If we want to have the window destroyed and it has already been closed by the user,
	//     the window is only hidden and we can go ahead and actually destroy it.
	if ( window->closed ) {
		if ( !DestroyWindow( window->handle ) )
			BW_WIN32_ASSERT_ERROR;
	}
	// If the window is still active, but it isn't needed anymore in the code, we just activate automatic destruction:
	else {
		window->destroy_on_close = true;
	}
}

void bw_Window_dispatch( bw_Window* window, bw_WindowDispatchFn f, void* data ) {

	bw_WindowDispatchData* param = malloc( sizeof( bw_WindowDispatchData ) );
	param->func = f;
	param->window = window;
	param->data = data;

	// WM_APP + 1 is the message code for dispatching a window function
	if ( !PostThreadMessageW( window->app->thread_id, WM_APP + 1, 0, (LPARAM)param ) ) {
		free( param );

		BW_WIN32_ASSERT_ERROR;
	}
}

BOOL CALLBACK bw_Window_hideChild( HWND handle, LPARAM _ ) {

	ShowWindow( handle, SW_HIDE );

	return true;
}

LRESULT CALLBACK bw_Window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {

	bw_Window* window = (bw_Window*)GetWindowLongPtrW( hwnd, GWLP_USERDATA );

	switch (msg) {
	case WM_SIZE:
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
		break;
	// When closing the window, only destroy it when it is ready for it to be destroyed
	case WM_CLOSE:
		if ( window->destroy_on_close )
			DestroyWindow( hwnd );
		else
			bw_Window_close( window );
		break;
	case WM_NCDESTROY:
		bw_Window_free( window );
		break;
	default:
		return DefWindowProcW(hwnd, msg, wp, lp);
	}

	return 0;
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
	window->closed = false;
	window->destroy_on_close = false;
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
		L"browser_window",
		title,
		window_style,
		0, 0,
		width,
		height,
		(parent == 0 ? HWND_DESKTOP : parent->handle),
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

	// Increase the window counter
	app->windows_alive += 1;

	return window;
}
