#ifndef BW_BROWSER_WINDOW_CEF_H
#define BW_BROWSER_WINDOW_CEF_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>



struct bw_BrowserWindowInner {
	void* cef_ptr;
	bool handle_is_used;
};



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_CEF_H
