#ifndef BW_BROWSER_WINDOW_IMPL_H
#define BW_BROWSER_WINDOW_IMPL_H

#ifdef __cplusplus
extern "C" {
#endif



void bw_BrowserWindowImpl_clean(bw_BrowserWindowImpl* bw);

// Should be implemented by the underlying browser engine to create a new browser and invoke the callback.
void bw_BrowserWindowImpl_new(
	bw_BrowserWindow* browser,
	bw_BrowserWindowSource source,
	int width, int height,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,
	void* callback_data
);

void bw_BrowserWindowImpl_onResize( const bw_Window* window, unsigned int width, unsigned int height );



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_IMPL_H
