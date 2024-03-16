#ifndef BW_WINDOW_WIN32_H
#define BW_WINDOW_WIN32_H

#ifdef __cplusplus
extern "C" {
#endif

#include "../win32.h"
#include "../bool.h"


typedef struct bw_Window bw_Window;
typedef void (*bw_WindowDispatchFn)( bw_Window* window, void* data );


struct bw_WindowDispatchData {
	bw_WindowDispatchFn func;
	bw_Window* window;
	void* data;

};

typedef struct {
	void* handle;
	BOOL closed;
	DWORD style;
	BYTE opacity;
} bw_WindowImpl;


void bw_WindowWin32_onResize(bw_Window*, int left, int right, int top, int bottom);


#ifdef __cplusplus
}
#endif

#endif//BW_WINDOW_WIN32_H
