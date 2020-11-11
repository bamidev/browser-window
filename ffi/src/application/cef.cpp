#include "../application.h"
#include "../debug.h"
#include "../cef/app_handler.hpp"
#include "../cef/client_handler.hpp"

#include "impl.h"

#include <include/cef_app.h>
#include <include/cef_base.h>
#include <stdlib.h>

// Link with win32 libraries
#if defined(BW_WIN32)
#pragma comment(lib, "shell32.lib")
#pragma comment(lib, "user32.lib")
#endif

// Causes the current process to exit with the given exit code.
void _bw_Application_exitProcess( int exit_code );



/*class bw_ApplicationDispatchTask: public CefTask {
	bw_Application* app;
	bw_ApplicationDispatchFn func;
	void* data;

public:
	bw_ApplicationDispatchTask( bw_Application* app, bw_ApplicationDispatchFn func, void* data ) :
		app(app), func(func), data(data)	{}

	void Execute() override {
		this->func( this->app, data );
	}

private:
	IMPLEMENT_REFCOUNTING( bw_ApplicationDispatchTask );
};

void _bw_Application_dispatch_exit( bw_Application* app, void* data ) {
	int* param = (int*)data;

	bw_Application_exit( app, *param );

	delete param;
}

void bw_Application_dispatch( bw_Application* app, bw_ApplicationDispatchFn func, void* data ) {
	CefRefPtr<bw_ApplicationDispatchTask> task( new bw_ApplicationDispatchTask( app, func, data ) );
	CefPostTask( TID_UI, task.get() );
}

void bw_Application_exit( bw_Application* app, int exit_code ) {
	app->exit_code = exit_code;

	CefQuitMessageLoop();
}*/

/*void bw_Application_exit_async( bw_Application* app, int exit_code ) {
	int* param = new int( exit_code );

	// This will call bw_Application_exit, but on the GUI thread
	bw_Application_dispatch( app, _bw_Application_dispatch_exit, (void*)param );
}*/

bw_ApplicationEngineImpl bw_ApplicationEngineImpl_start( bw_Application* app, int argc, char** argv ) {
	bw_ApplicationEngineImpl impl;

	// For some reason the Windows implementation for CEF doesn't have the constructor for argc and argv.
#ifdef BW_WIN32
	CefMainArgs main_args( GetModuleHandle(NULL) );
#else
	CefMainArgs main_args( argc, argv );
#endif

	CefSettings app_settings;
	// Only works on Windows:
#ifdef BW_WIN32
	app_settings.multi_threaded_message_loop = true;
#endif

	CefBrowserSettings browser_settings;

	CefRefPtr<CefApp> cef_app_handle( new AppHandler( app ) );

	int exit_code = CefExecuteProcess( main_args, cef_app_handle.get(), 0 );

	// If the current process returns a non-negative number, it is not the main process on which we run user code.
	if ( exit_code >= 0 ) {
		exit( exit_code );
		return impl;
	}

	CefRefPtr<CefClient>* client = new CefRefPtr<CefClient>(new ClientHandler( app ));
	//client_handler = (ClientHandler*) client.get();

	CefInitialize( main_args, app_settings, cef_app_handle.get(), 0 );

	impl.exit_code = 0;
	impl.cef_client = (void*)client;

	return impl;
}

void bw_ApplicationEngineImpl_finish( bw_ApplicationEngineImpl* app ) {
	delete (CefRefPtr<CefClient>*)app->cef_client;
}

/*int bw_Application_run( bw_Application* app ) {
	CefRunMessageLoop();

	return app->exit_code;
}

void bw_Application_free( bw_Application* app ) {
	CefShutdown();

	CefRefPtr<CefClient>* app_cef = (CefRefPtr<CefClient>*)app->cef_client;
	delete app_cef;
	delete app;
}*/
