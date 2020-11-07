#include "../common.h"
#include "../window.h"

#include "impl.h"

#include <stdlib.h>



void bw_Window_destroy( bw_Window* window ) {

	// Check and see if we need to destroy our parent as well
	if ( window->parent != 0 && window->parent->dropped && window->parent->closed )
		bw_Window_destroy( window->parent );

	// Call cleanup handler
	if ( window->callbacks.do_cleanup != 0 )
		window->callbacks.do_cleanup( window );

	// Actually destroy and free our window
	bw_WindowImpl_destroy( window );
	free( window );

	// Decrease the window counter
	bw_Application* app = window->app;
	app->windows_alive -= 1;

	// Exit application if this was our last window
	if ( app->windows_alive == 0 )
		bw_Application_exit( app, 0 );
}



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
		bw_Window_destroy( window );
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
		bw_Window_destroy( window );
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
	bw_Application_checkThread( app );

	bw_Window* window = (bw_Window*)malloc( sizeof(bw_Window) );

	window->app = app;
	window->parent = parent;
	window->closed = false;
	window->dropped = false;
	window->user_data = user_data;
	memset( &window->callbacks, 0, sizeof( window->callbacks ) );

	window->impl = bw_WindowImpl_new( window, title, width, height, options );

	return window;
}

void bw_Window_open( bw_Window* window ) {
	window->closed = false;

	bw_WindowImpl_show( window );
}
