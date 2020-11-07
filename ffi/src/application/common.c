#include "../application.h"
#include "../common.h"

#include "impl.h"

#include <stdlib.h>



void bw_Application_free( bw_Application* app ) {

	bw_ApplicationEngineImpl_finish( &app->engine_impl );
	bw_ApplicationImpl_finish( &app->impl );
	free( app );
}

bw_Application* bw_Application_start( int argc, char** argv ) {

    bw_Application* app = (bw_Application*)malloc( sizeof( bw_Application ) );
    app->windows_alive = 0;

    app->engine_impl = bw_ApplicationEngineImpl_start( app, argc, argv );
	app->impl = bw_ApplicationImpl_start( app, argc, argv );

    return app;
}

void bw_Application_dispatch( bw_Application* app, bw_ApplicationDispatchFn func, void* data ) {

    bw_ApplicationDispatchData* dispatch_data = (bw_ApplicationDispatchData*)malloc( sizeof(bw_ApplicationDispatchData) );
    dispatch_data->func = func;
    dispatch_data->data = data;

    bw_ApplicationImpl_dispatch( app, dispatch_data );
}
