#include "../browser_window.h"
#include "../common.h"

#include "impl.h"


void bw_BrowserWindow_free( bw_BrowserWindow* bw ) {
	bw_BrowserWindowImpl_clean(&bw->impl);
	//free(bw);
}

bw_Application* bw_BrowserWindow_getApp( bw_BrowserWindow* bw ) {
	return bw->window->app;
}

bw_Window* bw_BrowserWindow_getWindow( bw_BrowserWindow* bw ) {
    return bw->window;
}

bw_BrowserWindow* bw_BrowserWindow_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice title,
	int width, int height,
	const bw_WindowOptions* window_options
) {
	bw_Application_assertCorrectThread( app );

	bw_BrowserWindow* browser = (bw_BrowserWindow*)malloc( sizeof( bw_BrowserWindow ) );
	browser->window = bw_Window_new( app, parent, title, width, height, window_options );
	browser->window->browser = browser;
	memset(&browser->events, 0, sizeof(bw_BrowserWindowEvents));
	return browser;
}

void bw_BrowserWindow_create(
	bw_BrowserWindow* bw,
	int width, int height,
	bw_BrowserWindowSource source,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,	// A function that gets invoked when the browser window has been created.
	void* callback_data	// Data that will be passed to the creation callback
) {
	bw_BrowserWindowImpl_new(
		bw,
		source,
		width,
		height,
		browser_window_options,
		callback,
		callback_data
	);
}
