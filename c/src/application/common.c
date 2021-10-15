#include "../application.h"
#include "../common.h"

#include "impl.h"

#include <stdlib.h>



void bw_Application_free( bw_Application* app ) {
	free( app );
}

void bw_Application_markAsDone(bw_Application* app) { printf("bw_Application_markAsDone %i\n", app->windows_alive);
	if (app->windows_alive == 0)
		bw_Application_exit(app, 0);
}

void bw_Application_runOnReady(bw_Application* app, void* user_data) {
	bw_ApplicationImpl_ReadyHandlerData* ready_handler_data = (bw_ApplicationImpl_ReadyHandlerData*)user_data;

	ready_handler_data->func(app, ready_handler_data->data);
}

int bw_Application_run( bw_Application* app, bw_ApplicationReadyFn on_ready, void* user_data ) {
	bw_Application_assertCorrectThread( app );

	bw_ApplicationImpl_ReadyHandlerData ready_handler_data = {
		app,
		on_ready,
		user_data
	};

	bw_ApplicationImpl_ReadyHandlerData handler_data_wrapper = {
		app,
		bw_Application_runOnReady,
		(void*)&ready_handler_data
	};

	return bw_ApplicationImpl_run( app, &handler_data_wrapper );
}

void bw_Application_finish( bw_Application* app ) {

	bw_ApplicationEngineImpl_finish( &app->engine_impl );
	bw_ApplicationImpl_finish( &app->impl );
}

bw_Err bw_Application_initialize( bw_Application** app, int argc, char** argv, const bw_ApplicationSettings* settings ) {

	*app = (bw_Application*)malloc( sizeof( bw_Application ) );
	(*app)->windows_alive = 0;
	(*app)->finished = FALSE;

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

BOOL bw_Application_dispatchDelayed( bw_Application* app, bw_ApplicationDispatchFn func, void* data, uint64_t milliseconds ) {

	bw_ApplicationDispatchData* dispatch_data = (bw_ApplicationDispatchData*)malloc( sizeof(bw_ApplicationDispatchData) );
	dispatch_data->func = func;
	dispatch_data->data = data;

	return bw_ApplicationImpl_dispatchDelayed( app, dispatch_data, milliseconds );
}
