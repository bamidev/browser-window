#include "../application.h"
#include "../debug.h"
#include "../cef/app_handler.hpp"
#include "../cef/client_handler.hpp"

#include "impl.h"

#include <include/cef_app.h>
#include <include/cef_base.h>
#include <stdlib.h>

// X11 headers, when used by CEF
#if defined(CEF_X11)
#include <X11/Xlib.h>
#endif

// Link with win32 libraries
#if defined(BW_WIN32)
#pragma comment(lib, "shell32.lib")
#endif

// Causes the current process to exit with the given exit code.
void _bw_Application_exitProcess( int exit_code );
CefString to_string( bw_CStrSlice );

#ifdef CEF_X11
int _bw_ApplicationCef_xErrorHandler( Display* display, XErrorEvent* event );
int _bw_ApplicationCef_xIoErrorHandler( Display* display );
#endif



bw_ApplicationEngineImpl bw_ApplicationEngineImpl_initialize( bw_Application* app, int argc, char** argv, const bw_ApplicationSettings* settings ) {
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

	// If working with X, set error handlers that spit out errors instead of shutting down the application
#if defined(CEF_X11)
	XSetErrorHandler( _bw_ApplicationCef_xErrorHandler );
	XSetIOErrorHandler( _bw_ApplicationCef_xIoErrorHandler );
#endif

	CefSettings app_settings;
	// Only works on Windows and Linux according to docs.
	// Here it says it works on Windows only: https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage.md#markdown-header-linux
	// TODO: Check if the GTK implementation works when it is set to false, and with CefMessageDoWork() called repeatedly from somewhere else.
	// TODO: Check if it works on BSD by any chance.
	// TODO: For unsupported systems (like macOS), CefDoMessageLoopWork needs to be called repeatedly.
	//       This is usually less effecient than using the multithreaded message loop though.
	// TODO: If GTK will be used on macOS in the future, the 'if' macro below needs to be corrected.
#if defined(BW_WIN32) || defined(BW_GTK)
	app_settings.multi_threaded_message_loop = true;
#endif
	if ( settings->resource_dir.data != 0 ) {
		char* path = bw_string_copyAsNewCstr( settings->resource_dir );
		CefString( &app_settings.resources_dir_path ) = path;
		bw_string_freeCstr(path);
	}

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



#ifdef CEF_X11
int _bw_ApplicationCef_xErrorHandler( Display* display, XErrorEvent* event ) {

	fprintf( stderr, "X Error: type %d, serial %lu, error code %d, request code %d, mino	r code %d\n", event->type, event->serial, event->error_code, event->request_code, event->minor_code );
	return 0;
}

int _bw_ApplicationCef_xIoErrorHandler( Display* display ) {
	return 0;
}
#endif