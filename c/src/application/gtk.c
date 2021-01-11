#include "../application.h"

#include "impl.h"
#include "../common.h"

#include <gtk/gtk.h>



typedef struct {
	bw_Application* app;
	int exit_code;
} bw_ApplicationGtkAsyncExitData;

typedef struct {
	bw_Application* app;
	bw_ApplicationDispatchData* inner;
} bw_ApplicationImplDispatchData;



gboolean _bw_ApplicationImpl_dispatchHandler( gpointer _dispatch_data );
gboolean _bw_ApplicationImpl_exitHandler( gpointer data );



void bw_Application_assertCorrectThread( const bw_Application* app ) {
#ifndef NDEBUG
	BW_ASSERT( app->impl.thread_id == pthread_self(), "Browser Window C function called from non-GUI thread!" )
#else
	UNUSED(app);
#endif
}

void bw_Application_exit( bw_Application* app, int exit_code ) {
	app->impl.exit_code = exit_code;

	// Set `is_running` flag to false
	pthread_mutex_lock( &app->impl.is_running_mtx );
	app->impl.is_running = false;
	pthread_mutex_unlock( &app->impl.is_running_mtx );

	// Then quit the loop
	g_application_quit( G_APPLICATION( app->impl.handle ) );
}

void bw_Application_exitAsync( bw_Application* app, int exit_code ) {
	bw_ApplicationGtkAsyncExitData data;
	data.app = app;
	data.exit_code = exit_code;

	gdk_threads_add_idle( _bw_ApplicationImpl_exitHandler, (gpointer)&data );
}

void bw_ApplicationGtk_onActivate( GtkApplication* gtk_handle, gpointer data ) {
	UNUSED( gtk_handle );

	bw_ApplicationImpl_ReadyHandlerData* ready_handler_data = (bw_ApplicationImpl_ReadyHandlerData*)data;
	bw_ApplicationImpl* app = &ready_handler_data->app->impl;

	// Mark the application as 'running'
	pthread_mutex_lock( &app->is_running_mtx );
	app->is_running = true;
	pthread_mutex_unlock( &app->is_running_mtx );

	(ready_handler_data->func)( ready_handler_data->app, ready_handler_data->data );
}

int bw_ApplicationImpl_run( bw_Application* app, bw_ApplicationImpl_ReadyHandlerData* ready_handler_data ) {

	g_signal_connect( app->impl.handle, "activate", G_CALLBACK( bw_ApplicationGtk_onActivate ), (void*)ready_handler_data );

	g_application_run( G_APPLICATION(app->impl.handle), app->impl.argc, app->impl.argv );

	return app->impl.exit_code;
}

BOOL bw_ApplicationImpl_dispatch( bw_Application* app, bw_ApplicationDispatchData* _data ) {
	BOOL is_running = true;
	
	pthread_mutex_lock( &app->impl.is_running_mtx );

	bw_ApplicationImplDispatchData* data = (bw_ApplicationImplDispatchData*)malloc( sizeof( bw_ApplicationImplDispatchData ) );
	data->app = app;
	data->inner = _data;

	if ( app->impl.is_running )
		gdk_threads_add_idle( _bw_ApplicationImpl_dispatchHandler, (gpointer)data );
	else
		is_running = false;

	pthread_mutex_unlock( &app->impl.is_running_mtx );

	return is_running;
}

bw_ApplicationImpl bw_ApplicationImpl_initialize( bw_Application* _app, int argc, char** argv, const bw_ApplicationSettings* settings ) {
	UNUSED( _app );
	UNUSED( settings );

	bw_ApplicationImpl app;

	app.handle = gtk_application_new("bamilab.BrowserWindow", G_APPLICATION_FLAGS_NONE);
	app.argc = argc;
	app.argv = argv;
	app.is_running = false;
	app.thread_id = pthread_self();

	// Initialize mutex
	int result = pthread_mutex_init( &app.is_running_mtx, NULL );
	BW_POSIX_ASSERT_SUCCESS( result );

	return app;
}

// There is no 'free' function for GtkApplication*
void bw_ApplicationImpl_finish( bw_ApplicationImpl* app ) {

	pthread_mutex_destroy( &app->is_running_mtx );
	g_object_unref( app->handle );
}



gboolean _bw_ApplicationImpl_dispatchHandler( gpointer _dispatch_data ) {
	bw_ApplicationImplDispatchData* dispatch_data = (bw_ApplicationImplDispatchData*)(_dispatch_data);

	dispatch_data->inner->func( dispatch_data->app, dispatch_data->inner->data );

	free( dispatch_data->inner );
	free( dispatch_data );

	return FALSE;
}

gboolean _bw_ApplicationImpl_exitHandler( gpointer _data ) {
	bw_ApplicationGtkAsyncExitData* data = (bw_ApplicationGtkAsyncExitData*)_data;

	bw_Application_exit( data->app, data->exit_code );

	return FALSE;
}
