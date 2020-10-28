#include "../browser_window.h"



void bw_BrowserWindow_close( bw_BrowserWindow* bw ) {
	bw_Window_close( bw->window );
}

void bw_BrowserWindow_drop( bw_BrowserWindow* bw ) {
	bw_Window_free( bw->window );
	free( bw );
}
