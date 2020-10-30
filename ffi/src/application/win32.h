#ifndef BW_APPLICATION_WIN32_H
#define BW_APPLICATION_WIN32_H

#ifdef BW_CEF
#include "cef.h"
#else
#error Unsupported browser engine selected
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define WIN32_LEAN_AND_MEAN
#include <Windows.h>



struct bw_Application {
	DWORD thread_id;
	HINSTANCE handle;
	WNDCLASSEX wc;
	struct bw_ApplicationEngineData* engine_data;	/// Can be set by the implementation of a browser engine
};

struct bw_ApplicationDispatchData {
	bw_ApplicationDispatchFn func;
	void* data;
};



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_WIN32_H
