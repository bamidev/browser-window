#ifndef BW_APPLICATION_CEF_H
#define BW_APPLICATION_CEF_H

#ifdef __cplusplus
extern "C" {
#endif



struct bw_ApplicationEngineData {
	void* cef_client;
	int exit_code;
};

/*struct bw_ApplicationDispatchData {
	bw_ApplicationDispatchFn func;
	void* data;
};*/



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_CEF_H
