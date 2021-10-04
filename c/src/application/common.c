#include "../application.h"
#include "../common.h"

#include "impl.h"

#include <stdlib.h>



void bw_Application_free( bw_Application* app ) {
	free( app );
}

int bw_Application_run( bw_Application* app, bw_ApplicationReadyFn on_ready, void* user_data ) {
	bw_Application_assertCorrectThread( app );

	bw_ApplicationImpl_ReadyHandlerData ready_handler_data = {
		app,
		on_ready,
		user_data
	};

	return bw_ApplicationImpl_run( app, &ready_handler_data );
}

void bw_Application_finish( bw_Application* app ) {

	bw_ApplicationEngineImpl_finish( &app->engine_impl );
	bw_ApplicationImpl_finish( &app->impl );
}

bw_Err bw_Application_initialize( bw_Application** app, int argc, char** argv, const bw_ApplicationSettings* settings ) {

	*app = (bw_Application*)malloc( sizeof( bw_Application ) );
	(*app)->windows_alive = 0;

	bw_Err error = bw_ApplicationEngineImpl_initialize( &(*app)->engine_impl, (*app), argc, argv, settings );
	if (BW_ERR_IS_FAIL(error))	return error;
	(*app)->impl = bw_ApplicationImpl_initialize( (*app), argc, argv, settings );

	BW_ERR_RETURN_SUCCESS;
}

BOOL bw_Application_dispatch( bw_Application* app, bw_ApplicationDispatchFn func, void* data ) {

	bw_ApplicationDispatchData* dispatch_data = (bw_ApplicationDispatchData*)malloc( sizeof(bw_ApplicationDispatchData) );
	dispatch_data->func = func;
	dispatch_data->data = data;

	return bw_ApplicationImpl_dispatch( app, dispatch_data );
}
