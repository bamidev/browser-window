#include "../application.h"

#include "impl.h"
#include "../common.h"

#include <gtk/gtk.h>



typedef struct {
	bw_Application* app;
	int exit_code;
} bw_ApplicationGtkAsyncExitData;



gboolean _bw_ApplicationImpl_dispatchHandler( gpointer _dispatch_data );
gboolean _bw_ApplicationImpl_exitHandler( gpointer data );



void bw_Application_checkThread( const bw_Application* app ) {
#ifndef NDEBUG
	// TODO: Check if called from the correct thread

#else
	UNUSED(app);
#endif
}

void bw_Application_exit( bw_Application* app, int exit_code ) {
	app->impl.exit_code = exit_code;
	gtk_main_quit();
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

	(ready_handler_data->func)( ready_handler_data->app, ready_handler_data->data );
}

int bw_ApplicationImpl_run( bw_Application* app, bw_ApplicationImpl_ReadyHandlerData* ready_handler_data ) {

	g_signal_connect( app->impl.handle, "activate", G_CALLBACK( bw_ApplicationGtk_onActivate ), (void*)ready_handler_data );

	g_application_run( G_APPLICATION(app->impl.handle), app->impl.argc, app->impl.argv );

	return app->impl.exit_code;
}

void bw_ApplicationImpl_dispatch( bw_Application* app, bw_ApplicationDispatchData* data ) {
	(void)(app);

	gdk_threads_add_idle( _bw_ApplicationImpl_dispatchHandler, (gpointer)data );
}

bw_ApplicationImpl bw_ApplicationImpl_start( bw_Application* _app, int argc, char** argv ) {
	(void)(_app);

	bw_ApplicationImpl app;

	app.handle = gtk_application_new("bamilab.BrowserWindow", G_APPLICATION_FLAGS_NONE);
	app.argc = argc;
	app.argv = argv;

	return app;
}

// There is no 'free' function for GtkApplication*
void bw_ApplicationImpl_finish( bw_ApplicationImpl* app ) {
	g_object_unref( app->handle );
}



gboolean _bw_ApplicationImpl_dispatchHandler( gpointer _dispatch_data ) {
	bw_ApplicationDispatchData* dispatch_data = (bw_ApplicationDispatchData*)(_dispatch_data);

	dispatch_data->func( dispatch_data->app, dispatch_data->data );

	return FALSE;
}

gboolean _bw_ApplicationImpl_exitHandler( gpointer _data ) {
	bw_ApplicationGtkAsyncExitData* data = (bw_ApplicationGtkAsyncExitData*)_data;

	bw_Application_exit( data->app, data->exit_code );

	return FALSE;
}
