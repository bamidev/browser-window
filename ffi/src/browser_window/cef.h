#ifndef BW_BROWSER_WINDOW_CEF_H
#define BW_BROWSER_WINDOW_CEF_H

#ifdef __cplusplus
extern "C" {
#endif



typedef struct {
	void* cef_ptr;
	char* resource_path;
} bw_BrowserWindowImpl;



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_CEF_H
