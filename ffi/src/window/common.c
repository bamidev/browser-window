#include "../window.h"

#include "impl.h"



const bw_Application* bw_Window_getApp( bw_Window* window ) {
	return window->app;
}

bool bw_Window_isClosed( const bw_Window* window ) {
	return window->closed;
}

// Closing a window hides the window,
//  and if the window has been dropped, it will be destroyed.
// This should also be called from the window implementations close event.
void bw_Window_close( bw_Window* window ) {
	window->closed = true;

	if ( window->dropped ) {
		bw_WindowImpl_destroy( window );
		free( window );
	}
	else
		bw_WindowImpl_hide( window );

	// TODO: Fire on_closed event
}

// Dropping the window handle may destroy if the window has also been closed.
// Otherwise it will be destroyed on close.
void bw_Window_drop( bw_Window* window ) {
	window->dropped = true;

	if ( window->closed ) {
		bw_WindowImpl_destroy( window );
		free( window );
	}
}

bw_Window* bw_Window_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice title,
	int width, int height,
	const bw_WindowOptions* options,
	void* user_data
) {
	bw_Window* window = (bw_Window*)malloc( sizeof(bw_Window) );

	window->app = app;
	window->parent = parent;
	window->closed = false;
	window->dropped = false;
	window->user_data = user_data;

	window->impl = bw_WindowImpl_new( window, title, width, height, options );

	memset( &window->callbacks, 0, sizeof( window->callbacks ) );

	return window;
}

void bw_Window_open( bw_Window* window ) {
	window->closed = false;

	bw_WindowImpl_show( window );
}
