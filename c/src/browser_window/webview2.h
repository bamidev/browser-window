#ifndef BW_BROWSER_WINDOW_WEBVIEW2_H
#define BW_BROWSER_WINDOW_WEBVIEW2_H

#ifdef __cplusplus
extern "C" {
#endif



struct bw_BrowserWindowInner {
	void* webview;	/* Type: ICoreWebView2 */
	void* webview_controller;	/* Type: ICoreWebView2Controller */
};



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_WEBVIEW2_H
