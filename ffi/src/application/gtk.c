#include "../application.h"

#include "impl.h"



typedef struct {
	bw_Application* app;
	int exit_code;
} bw_ApplicationGtkAsyncExitData;



gboolean _bw_ApplicationImpl_dispatchHandler( gpointer _dispatch_data );
gboolean _bw_ApplicationImpl_exitHandler( gpointer data );



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

int bw_Application_run( bw_Application* app ) {
	gtk_main();
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
	(void)(app);
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
