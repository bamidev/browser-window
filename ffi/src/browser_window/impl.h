#ifndef BW_BROWSER_WINDOW_IMPL_H
#define BW_BROWSER_WINDOW_IMPL_H

#ifdef __cplusplus
extern "C" {
#endif



void bw_BrowserWindowImpl_doCleanup( bw_Window* bw );

// Should be implemented by the underlying browser engine to create a new browser and invoke the callback.
bw_BrowserWindowImpl bw_BrowserWindowImpl_new(
	const bw_BrowserWindow* browser,
	bw_BrowserWindowSource source,
	int width, int height,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,
	void* callback_data
);



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_IMPL_H
