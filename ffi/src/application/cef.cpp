#pragma comment(lib, "libcef.lib")
#pragma comment(lib, "libcef_dll_wrapper.lib")

#include "../application.h"
#include "../cef/app_handler.hpp"
#include "../cef/client_handler.hpp"

#include <include/cef_app.h>
#include <include/cef_base.h>

#ifdef BW_CEF_WINDOWS
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#pragma comment(lib, "shell32.lib")
#pragma comment(lib, "user32.lib")
#endif

// Causes the current process to exit with the given exit code.
void _bw_Application_exit_process( int exit_code );



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

void _bw_Application_exit_process( int exit_code ) {
#ifdef BW_WIN32
	ExitProcess( exit_code );
#endif
}

/*void bw_Application_exit_async( bw_Application* app, int exit_code ) {
	int* param = new int( exit_code );

	// This will call bw_Application_exit, but on the GUI thread
	bw_Application_dispatch( app, _bw_Application_dispatch_exit, (void*)param );
}*/

void bw_Application_init( bw_Application* app ) {
	app->engine_data = new bw_ApplicationEngineData;

#ifdef BW_CEF_WINDOWS
	CefMainArgs main_args( GetModuleHandle( NULL ) );
#else
#error Platform not yet supported
#endif

	CefSettings app_settings;
	// Only works on Windows:
	app_settings.multi_threaded_message_loop = 1;
	CefBrowserSettings browser_settings;
	CefRefPtr<CefApp> cef_app_handle( new AppHandler( app ) );

	int exit_code = CefExecuteProcess( main_args, cef_app_handle.get(), 0 );
	// If the current process returns a non-negative number, it is not the main process on which we run user code.
	if ( exit_code >= 0 ) {
		_bw_Application_exit_process( exit_code );
		return;
	}

	CefRefPtr<CefClient>* client = new CefRefPtr<CefClient>(new ClientHandler( app ));
	//client_handler = (ClientHandler*) client.get();

	CefInitialize( main_args, app_settings, cef_app_handle.get(), 0 );

	app->engine_data->exit_code = 0;
	app->engine_data->cef_client = (void*)client;
}

void bw_Application_step() {
	CefDoMessageLoopWork();
}

void bw_Application_uninit( bw_Application* app ) {
	delete app->engine_data;
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
