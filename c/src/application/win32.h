#ifndef BW_APPLICATION_WIN32_H
#define BW_APPLICATION_WIN32_H

#if defined(BW_CEF)
#include "cef.h"
#elif defined(BW_EDGE)
#include "edge.h"
#else
#error Unsupported browser engine selected
#endif

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>



typedef struct {
	DWORD thread_id;
	HINSTANCE handle;
	WNDCLASSEXW wc;
	bool is_running;
	SRWLOCK is_running_mtx;
} bw_ApplicationImpl;

struct bw_ApplicationDispatchData {
	bw_ApplicationDispatchFn func;
	void* data;
};



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_WIN32_H
