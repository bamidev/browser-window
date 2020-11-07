#ifndef BW_APPLICATION_CEF_H
#define BW_APPLICATION_CEF_H

#ifdef __cplusplus
extern "C" {
#endif



typedef struct {
	void* cef_client;
	int exit_code;
} bw_ApplicationEngineImpl;



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_CEF_H
