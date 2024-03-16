#include "../browser_window.h"
#include "impl.h"


void bw_BrowserWindowImpl_clean(bw_BrowserWindowImpl* bw) {
	UNUSED(bw);
}

void bw_BrowserWindowImpl_new(
	bw_BrowserWindow* browser,
	bw_BrowserWindowSource source,
	int width, int height,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,
	void* callback_data
) {
	browser->impl.controller = NULL;
	browser->impl.webview = NULL;
}
