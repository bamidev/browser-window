#ifndef BW_APPLICATION_COMMON_H
#define BW_APPLICATION_COMMON_H

#ifdef __cplusplus
extern "C" {
#endif

#include "../application.h"

typedef struct {
	bw_Application* app;
	bw_ApplicationReadyFn func;
	void* data;
} bw_ApplicationImpl_ReadyHandlerData;




bool bw_ApplicationImpl_dispatch( bw_Application* app, bw_ApplicationDispatchData* data );
void bw_ApplicationImpl_finish( bw_ApplicationImpl* );
int bw_ApplicationImpl_run( bw_Application* app, bw_ApplicationImpl_ReadyHandlerData* ready_handler_data );
bw_ApplicationImpl bw_ApplicationImpl_start( bw_Application* app, int argc, char** argv );

void bw_ApplicationEngineImpl_finish( bw_ApplicationEngineImpl* );
bw_ApplicationEngineImpl bw_ApplicationEngineImpl_start( bw_Application* app, int argc, char** argv );



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_APPLICATION_COMMON_H
