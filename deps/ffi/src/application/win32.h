#ifndef BW_APPLICATION_WIN32_H
#define BW_APPLICATION_WIN32_H

#include "../application.h"

#ifdef __cplusplus
extern "C" {
#endif

#define WIN32_LEAN_AND_MEAN
#include <Windows.h>



struct bw_Application {
	DWORD thread_id;
	HINSTANCE handle;
	WNDCLASSEX wc;
	void* engine_data;	/// Can be set by the implementation of a browser engine
};

struct bw_ApplicationDispatchData {
	bw_ApplicationDispatchFn func;
	void* data;
};



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_WIN32_H
