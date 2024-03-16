#include "../common.h"
#include "../window.h"

#include "impl.h"

#include <stdlib.h>
#include <string.h>



void bw_Window_close(bw_Window* window) {
	if (window->user_data != NULL) {
		bw_Window_freeUserData(window);
	}
	bw_WindowImpl_close(&window->impl);
}

void bw_Window_free( bw_Window* window ) {

	// Free the memory
	bw_Application* app = window->app;
	free( window );
	// Decrease the window counter and exit if we are done and all windows are closed.
	app->windows_alive -= 1;
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

bw_Window* bw_Window_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice title,
	int width, int height,
	const bw_WindowOptions* options
) {
	bw_Application_assertCorrectThread( app );

	bw_Window* window = (bw_Window*)malloc( sizeof(bw_Window) );

	window->app = app;
	window->parent = parent;
	window->closed = true;  // Windows start out hidden to the user
	window->dropped = false;
	window->user_data = NULL;
	window->browser = NULL;

	window->impl = bw_WindowImpl_new( window, title, width, height, options );

	app->windows_alive += 1;

	return window;
}

void bw_Window_show( bw_Window* window ) {
	window->closed = false;

	bw_WindowImpl_show( &window->impl );
}
