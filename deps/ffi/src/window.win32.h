#ifndef BW_WINDOWWIN32_H
#define BW_WINDOWWIN32_H

#ifdef __cplusplus
extern "C" {
#endif

// Windows.h defines somewhere that HWND eventually evualates to void*
#include <Windows.h>



struct bw_WindowInner {
	HWND handle;
	bool destroy_on_close;	// When true, the window can be destroyed (cleaned up) when it is closed by the user
};



#ifdef __cplusplus
}	// extern "C"
#endif

#endif//BW_WINDOWWIN32_H
