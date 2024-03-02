#ifndef BW_WINDOW_WIN32_H
#define BW_WINDOW_WIN32_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>

// Some definitions as defined in windef.h:
// https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types
// Because including windef.h with MinGW can cause some issues
typedef unsigned char BYTE;
typedef unsigned long DWORD;


typedef struct bw_Window bw_Window;
typedef void (*bw_WindowDispatchFn)( bw_Window* window, void* data );


struct bw_WindowDispatchData {
	bw_WindowDispatchFn func;
	bw_Window* window;
	void* data;

};

typedef struct {
	void* handle;
	DWORD style;
	BYTE opacity;
} bw_WindowImpl;


#ifdef __cplusplus
}
#endif

#endif//BW_WINDOW_WIN32_H
