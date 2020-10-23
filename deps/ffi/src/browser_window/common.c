#include "../browser_window.h"



extern void bw_BrowserWindow_doCleanup( bw_Window* w );
void bw_BrowserWindow_onLoad( bw_Window* w );
void bw_BrowserWindow_onClose( bw_Window* w );
void bw_BrowserWindow_onDestroy( bw_Window* w );
void bw_BrowserWindow_onLoaded( bw_Window* w );



const bw_Application* bw_BrowserWindow_get_app( bw_BrowserWindow* bw ) {
	return bw->window->app;
}

void* bw_BrowserWindow_get_user_data( bw_BrowserWindow* bw ) {
	return bw->user_data;
}

void _bw_BrowserWindow_init_window_callbacks( bw_BrowserWindow* bw ) {

	bw->callbacks.do_cleanup = bw_BrowserWindow_doCleanup;
	bw->callbacks.on_close = bw_BrowserWindow_onClose;
	bw->callbacks.on_destroy = bw_BrowserWindow_onDestroy;
	bw->callbacks.on_loaded = bw_BrowserWindow_onLoaded;
}

void bw_BrowserWindow_doCleanup( bw_Window* w ) {
	bw_BrowserWindow* bw = (bw_BrowserWindow*)w->user_data;
	bw->callbacks.do_cleanup( bw );

	// Perform some browser engine cleanup
	_bw_BrowserWindow_doCleanup( bw );
}

void bw_BrowserWindow_onClose( bw_Window* w ) {
	bw_BrowserWindow* bw = (bw_BrowserWindow*)w->user_data;
	bw->callbacks.on_close( bw );
}

void bw_BrowserWindow_onDestroy( bw_Window* w ) {
	bw_BrowserWindow* bw = (bw_BrowserWindow*)w->user_data;
	bw->callbacks.on_destroy( bw );
}

void bw_BrowserWindow_onLoaded( bw_Window* w ) {
	bw_BrowserWindow* bw =(bw_BrowserWindow*)w->user_data;
	bw->callbacks.on_loaded( bw );
}
