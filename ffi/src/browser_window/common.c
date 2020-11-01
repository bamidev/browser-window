#include "../browser_window.h"



void bw_BrowserWindow_onLoad( bw_Window* w );
void bw_BrowserWindow_onClose( bw_Window* w );
void bw_BrowserWindow_onDestroy( bw_Window* w );
void bw_BrowserWindow_onLoaded( bw_Window* w );



void bw_BrowserWindow_close( bw_BrowserWindow* bw ) {
	bw_Window_close( bw->window );
}

void bw_BrowserWindow_drop( bw_BrowserWindow* bw ) {

	// Let the window module know that the user has dropped the handle and doesn't use it anymore
	bw_Window_drop( bw->window );
}

const bw_Application* bw_BrowserWindow_getApp( bw_BrowserWindow* bw ) {
	return bw->window->app;
}

void* bw_BrowserWindow_getUserData( bw_BrowserWindow* bw ) {
	return bw->user_data;
}

void _bw_BrowserWindow_initWindowCallbacks( bw_BrowserWindow* bw ) {

	bw->callbacks.on_close = bw_BrowserWindow_onClose;
	bw->callbacks.on_loaded = bw_BrowserWindow_onLoaded;
}

void bw_BrowserWindow_onClose( bw_Window* w ) {
	bw_BrowserWindow* bw = (bw_BrowserWindow*)w->user_data;
	bw->callbacks.on_close( bw );
}

void bw_BrowserWindow_onLoaded( bw_Window* w ) {
	bw_BrowserWindow* bw =(bw_BrowserWindow*)w->user_data;
	bw->callbacks.on_loaded( bw );
}
