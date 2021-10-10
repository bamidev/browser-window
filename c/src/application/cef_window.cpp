#include "../application.h"
#include "../common.h"
#include "impl.h"

#include <include/cef_base.h>
#include <include/cef_app.h>
#include <include/base/cef_bind.h>
#include <include/wrapper/cef_closure_task.h>



void bw_Application_assertCorrectThread( const bw_Application* app ) {
	BW_ASSERT( CefCurrentlyOn( TID_UI ), "Not called from the GUI thread!" );
}

void bw_Application_exit( bw_Application* app, int exit_code ) {
	bw_Application_assertCorrectThread( app );

	app->impl.exit_code = exit_code;
	app->impl.is_running = false;

	CefQuitMessageLoop();
}

void bw_Application_exitAsync( bw_Application* app, int exit_code ) {
	CefPostTask( TID_UI, base::Bind( &bw_Application_exit, app, exit_code ));
}

void bw_ApplicationImpl_dispatchHandler( bw_Application* app, bw_ApplicationDispatchData* data ) {
	data->func( app, data->data );
}



BOOL bw_ApplicationImpl_dispatch( bw_Application* app, bw_ApplicationDispatchData* data ) {

	CefPostTask( TID_UI, base::Bind( &bw_ApplicationImpl_dispatchHandler, app, data ) );
	return true;
}

void bw_ApplicationImpl_finish( bw_ApplicationImpl* app ) {
	UNUSED( app );
	CefShutdown();
}

int bw_ApplicationImpl_run( bw_Application* app, bw_ApplicationImpl_ReadyHandlerData* ready_handler_data ) {
	bw_Application_assertCorrectThread( app );

	app->impl.exit_code = 0;
	app->impl.is_running = true;

	CefPostTask(TID_UI, base::Bind(ready_handler_data->func, app, ready_handler_data->data ));
	CefRunMessageLoop();

	return app->impl.exit_code;
}

// Doesn't need to be implemented because it is already done so in bw_ApplicationEngineImpl_initialize
bw_ApplicationImpl bw_ApplicationImpl_initialize( bw_Application* app, int argc, char** argv, const bw_ApplicationSettings* settings ) {
	UNUSED( app );
	UNUSED( argc );
	UNUSED( argv );
	UNUSED( settings );

	bw_ApplicationImpl impl;
	impl.is_running = false;
	return impl;
}


