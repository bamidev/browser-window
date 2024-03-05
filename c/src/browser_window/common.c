#include "../browser_window.h"
#include "../common.h"

#include "impl.h"




void bw_BrowserWindow_onLoad( bw_Window* w );
void bw_BrowserWindow_onDestroy( bw_Window* w );



void bw_BrowserWindow_destroy( bw_BrowserWindow* bw ) {
	bw_Window_destroy( bw->window );
}

void bw_BrowserWindow_drop( bw_BrowserWindow* bw ) {
	bw_Application_assertCorrectThread( bw->window->app );

	// Let the window module know that the user has dropped the handle and doesn't use it anymore
	bw_Window_drop( bw->window );
}

bw_Application* bw_BrowserWindow_getApp( bw_BrowserWindow* bw ) {
	return bw->window->app;
}

void* bw_BrowserWindow_getUserData( bw_BrowserWindow* bw ) {
	return bw->user_data;
}

bw_Window* bw_BrowserWindow_getWindow( bw_BrowserWindow* bw ) {
    return bw->window;
}

bw_BrowserWindow* bw_BrowserWindow_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice title,
	int width, int height,
	const bw_WindowOptions* window_options,
	bw_BrowserWindowHandlerFn handler,	/// A function that gets invoked when javascript the appropriate call is made in javascript.
	void* user_data	// The data that will be passed to the above handler function and the creation-callback when they are invoked.
) {
	bw_Application_assertCorrectThread( app );

	bw_BrowserWindow* browser = (bw_BrowserWindow*)malloc( sizeof( bw_BrowserWindow ) );
	browser->window = bw_Window_new( app, parent, title, width, height, window_options, browser );
	browser->window->callbacks.do_cleanup = bw_BrowserWindowImpl_doCleanup;
	browser->external_handler = handler;
	browser->user_data = user_data;
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

	// bw_BrowserWindowImpl_onResize depends on browser->impl being initialized already.
	// Therefore we initialize this event after everything
	bw->window->callbacks.on_resize = bw_BrowserWindowImpl_onResize;
}
