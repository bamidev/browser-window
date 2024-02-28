#ifndef BW_BROWSER_WINDOW_EDGE2_H
#define BW_BROWSER_WINDOW_EDGE2_H

#ifdef __cplusplus
extern "C" {
#endif


typedef struct {
	void* webview;	/* Type: ICoreWebView2 */
	void* webview_controller;	/* Type: ICoreWebView2Controller */
} bw_BrowserWindowImpl;


#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_EDGE2_H