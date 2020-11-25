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



bw_ApplicationEngineImpl bw_ApplicationEngineImpl_initialize( bw_Application* app, int argc, char** argv ) {
    bw_ApplicationEngineImpl impl;

	// For some reason the Windows implementation for CEF doesn't have the constructor for argc and argv.
#ifdef BW_WIN32
	CefMainArgs main_args( GetModuleHandle(NULL) );
#else
	CefMainArgs main_args( argc, argv );
#endif

	CefRefPtr<CefApp> cef_app_handle( new AppHandler( app ) );

	int exit_code = CefExecuteProcess( main_args, cef_app_handle.get(), 0 );

	// If the current process returns a non-negative number, it is not the main process on which we run user code.
	if ( exit_code >= 0 ) {
		exit( exit_code );
		return impl;
	}

	CefSettings app_settings;
	// Only works on Windows:
#ifdef BW_WIN32
	app_settings.multi_threaded_message_loop = true;
#endif

	CefInitialize( main_args, app_settings, cef_app_handle.get(), 0 );

	CefRefPtr<CefClient>* client = new CefRefPtr<CefClient>(new ClientHandler( app ));

	impl.exit_code = 0;
	impl.cef_client = (void*)client;

	return impl;
}

void bw_ApplicationEngineImpl_finish( bw_ApplicationEngineImpl* app ) {
    CefShutdown();
	delete (CefRefPtr<CefClient>*)app->cef_client;
}