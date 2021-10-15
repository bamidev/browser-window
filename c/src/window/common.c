#include "../common.h"
#include "../window.h"

#include "impl.h"

#include <stdlib.h>
#include <string.h>



void bw_Window_destroy( bw_Window* window ) {

	// Call cleanup handler
	if ( window->callbacks.do_cleanup != 0 )
		window->callbacks.do_cleanup( window );

	// Actually destroy our window
	bw_WindowImpl_destroy( &window->impl );

	// Free the memory
	free( window );

	// Decrease the window counter
	bw_Application* app = window->app;
	app->windows_alive -= 1;

	// Exit application if this was our last window
	if ( app->windows_alive == 0 )
		if (app->is_done) {
			bw_Application_exit( app, 0 ); }
}



const bw_Application* bw_Window_getApp( bw_Window* window ) {
	return window->app;
}

void bw_Window_hide( bw_Window* window ) {
	window->closed = true;

	bw_WindowImpl_hide( &window->impl );
}

bool bw_Window_isVisible( const bw_Window* window ) {
	return !window->closed;
}

// Dropping the window handle may destroy if the window has also been closed.
// Otherwise it will be destroyed on close.
void bw_Window_drop( bw_Window* window ) {
	window->dropped = true;

	if ( window->closed ) {
		bw_Window_destroy( window );
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
	bw_Application_assertCorrectThread( app );

	bw_Window* window = (bw_Window*)malloc( sizeof(bw_Window) );

	window->app = app;
	window->parent = parent;
	window->closed = true;  // Windows start out hidden to the user
	window->dropped = false;
	window->user_data = user_data;
	memset( &window->callbacks, 0, sizeof( window->callbacks ) );

	window->impl = bw_WindowImpl_new( window, title, width, height, options );

	app->windows_alive += 1;

	return window;
}

void bw_Window_show( bw_Window* window ) {
	window->closed = false;

	bw_WindowImpl_show( &window->impl );
}

// Closing a window hides the window,
//  and if the window has been dropped, it will be destroyed.
// This should also be called from the window implementations close event.
void bw_Window_triggerClose( bw_Window* window ) {
	window->closed = true;

	if ( window->dropped ) {
		bw_Window_destroy( window );
	}
	else {
		bw_WindowImpl_hide( &window->impl );
	}

	// TODO: Fire on_closed event
}