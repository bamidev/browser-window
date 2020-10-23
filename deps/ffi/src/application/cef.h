#ifndef BW_APPLICATION_WIN32_H
#define BW_APPLICATION_WIN32_H

#include "../application.h"

#ifdef __cplusplus
extern "C" {
#endif



struct bw_Application {
	void* cef_client;
	int exit_code;
};

struct bw_ApplicationDispatchData {
	bw_ApplicationDispatchFn func;
	void* data;
};



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_WIN32_H
