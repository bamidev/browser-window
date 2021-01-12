#include "../window.h"


#include "win32.h"
#include "../application/win32.h"
#include "../win32.h"
#include "../window.h"

#include <stdio.h>
#include <stdlib.h>



bw_Window* bw_Window_new(
	const bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options,
	void* user_data
) {
	bw_Window* w = malloc( sizeof( bw_Window ) );
	
	w->app = app;
	w->parent = parent;
	w->closed = false;
	w->user_data = user_data;

	memset( &w->callbacks, 0, sizeof( bw_WindowCallbacks ) );

	return w;
}

void bw_Window_close( bw_Window* window ) {}

void bw_Window_free( bw_Window* window ) {
	free( window );
}

void bw_Window_dispatch( bw_Window* window, bw_WindowDispatchFn f, void* data ) {}

void _bw_Window_init( bw_Application* app ) {}
