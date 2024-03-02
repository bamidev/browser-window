#include "../browser_window.h"
#include "impl.h"


void bw_BrowserWindowImpl_doCleanup( bw_Window* window ) {}

void bw_BrowserWindowImpl_new(
	bw_BrowserWindow* browser,
	bw_BrowserWindowSource source,
	int width, int height,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,
	void* callback_data
) {}

void bw_BrowserWindowImpl_onResize( const bw_Window* window, unsigned int width, unsigned int height ) {}
