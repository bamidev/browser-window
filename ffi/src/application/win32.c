#pragma comment(lib, "user32.lib")
#pragma comment(lib, "gdi32.lib")
#pragma comment(lib, "ole32.lib")

#include "../assert.h"
#include "../application.h"
#include "../common.h"

#include "impl.h"

#include <stdlib.h>
#define WIN32_LEAN_AND_MEAN
#include <WinDef.h>
#include <Windows.h>

#include "../win32.h"
#include "../window/win32.h"

#include <stdio.h>



void bw_Application_assertCorrectThread( const bw_Application* app ) {
#ifndef NDEBUG
	if ( app->impl.handle != GetModuleHandle(NULL) )
		BW_PANIC("Application handle used in invalid thread!");
#else
	UNUSED(app);
#endif
}

bool bw_ApplicationImpl_dispatch( bw_Application* app, bw_ApplicationDispatchData* dispatch_data ) {

	// Check if the runtime is still running
	AcquireSRWLockShared( &app->impl.is_running_mtx );
	bool result = app->impl.is_running;
	ReleaseSRWLockShared( &app->impl.is_running_mtx );
	if ( result == false )
		return false;

	PostThreadMessageW( app->impl.thread_id, WM_APP, (WPARAM)NULL, (LPARAM)dispatch_data );

	return true;
}


bool bw_Application_isRunning( const bw_Application* app ) {

	AcquireSRWLockShared( (SRWLOCK*)&app->impl.is_running_mtx );
	bool result = app->impl.is_running;
	ReleaseSRWLockShared( (SRWLOCK*)&app->impl.is_running_mtx );

	return result;
}


int bw_ApplicationImpl_run( bw_Application* app, bw_ApplicationImpl_ReadyHandlerData* ready_handler_data ) {

	MSG msg;
	BOOL res;
	int exit_code = 0;

	// We are ready immediately because all messages get queued anyway.
	AcquireSRWLockExclusive( &app->impl.is_running_mtx );
	app->impl.is_running = true;
	ReleaseSRWLockExclusive( &app->impl.is_running_mtx );

	(ready_handler_data->func)( app, ready_handler_data->data );

	bool exiting = false;
	while ( true ) {

		// When not exitting, just wait on messages normally.
		if ( !exiting ) {
			res = GetMessageW( &msg, 0, 0, 0 );

			// When WM_QUIT is received, turn on exiting mode
			if ( res == 0 ) {
				exit_code = (int)msg.wParam;
				exiting = true;

				// Set running flag to false
				AcquireSRWLockExclusive( &app->impl.is_running_mtx );
				app->impl.is_running = false;
				ReleaseSRWLockExclusive( &app->impl.is_running_mtx );
			}
		}
		// If exiting, don't wait on messages.
		// Only process those that are left for a graceful shutdown.
		else {
			res = PeekMessage( &msg, 0, 0, 0, PM_REMOVE );

			if ( res == 0 )
				break;
		}

		if (res == -1) {
			BW_WIN32_PANIC_LAST_ERROR;
		}
		else {
			TranslateMessage( &msg );
			DispatchMessageW( &msg );

			// Execute the dispatch functions when given
			if ( msg.message == WM_APP ) {
				bw_ApplicationDispatchData* params = (bw_ApplicationDispatchData*)msg.lParam;
				(params->func)( app, params->data );
				free( params );
			}
		}
	}

	// TODO: Wakeup all waiting delegation futures, so that they can return an error indiating that the runtime has exitted.
	UnregisterClassW( L"bw-window", app->impl.handle );

	return exit_code;
}


bw_ApplicationImpl bw_ApplicationImpl_initialize( bw_Application* _app, int argc, char** argv ) {
	UNUSED(_app);
	UNUSED(argc);
	UNUSED(argv);

	bw_ApplicationImpl app;
	app.is_running = false;
	InitializeSRWLock( &app.is_running_mtx );
	app.thread_id = GetCurrentThreadId();
	app.handle = GetModuleHandle( NULL );

	// Register window class
	memset( &app.wc, 0, sizeof(WNDCLASSEXW) );
	app.wc.cbSize = sizeof( WNDCLASSEXW );
	app.wc.hInstance = app.handle;
	app.wc.lpfnWndProc = bw_Window_proc;
	app.wc.lpszClassName = L"bw-window";
	RegisterClassExW( &app.wc );

	return app;
}


void bw_Application_exit( bw_Application* app, int exit_code ) {
	// We assume the size of an int is smaller or equal to the size of a pointer.
	// This should be true for 32 and 64 bit systems in general.
	_STATIC_ASSERT( sizeof(int) <= sizeof(WPARAM) );

	PostThreadMessageW( app->impl.thread_id, WM_QUIT, (WPARAM)exit_code, (LPARAM)NULL );
}

void bw_Application_exitAsync( bw_Application* app, int code ) {
	// PostThreadMessage is threadsafe, so we do exactly the same thing
	bw_Application_exit( app, code );
}

void bw_ApplicationImpl_finish( bw_ApplicationImpl* app ) {
	UNUSED(app);
}
