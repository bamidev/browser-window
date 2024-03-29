#ifndef BW_APPLICATION_WIN32_H
#define BW_APPLICATION_WIN32_H

#if defined(BW_CEF)
#include "cef.h"
#endif

#ifdef __cplusplus
extern "C" {
#endif

#include <windows.h>



typedef struct {
	DWORD thread_id;
	HINSTANCE handle;
	WNDCLASSEXW wc;
	SRWLOCK is_running_mtx;
} bw_ApplicationImpl;

typedef struct {
	void* dispatch_data;
	UINT delay;
	struct bw_Application* app;
} bw_ApplicationDispatchDelayedData;



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_WIN32_H
