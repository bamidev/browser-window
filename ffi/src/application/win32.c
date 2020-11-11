#pragma comment(lib, "user32.lib")
#pragma comment(lib, "gdi32.lib")
#pragma comment(lib, "ole32.lib")

#include "../assert.h"
#include "../application.h"
#include "../common.h"

#include <stdlib.h>
#define WIN32_LEAN_AND_MEAN
#include <WinDef.h>
#include <Windows.h>

#include "../win32.h"
#include "../window/win32.h"

#include <stdio.h>



void bw_Application_checkThread( const bw_Application* app ) {
#ifndef NDEBUG
	if ( app->impl.handle != GetModuleHandle(NULL) )
		BW_PANIC("Application handle used in invalid thread!");
#else
	UNUSED(app);
#endif
}

void bw_ApplicationImpl_dispatch( bw_Application* app, bw_ApplicationDispatchData* dispatch_data ) {
	PostThreadMessageW( app->impl.thread_id, WM_APP, (WPARAM)NULL, (LPARAM)dispatch_data );
}

int bw_ApplicationImpl_run( bw_Application* app, bw_ApplicationImpl_ReadyHandlerData* ready_handler_data ) {

	bw_Application_checkThread( app );

	MSG msg;
	int exit_code = 0;

	// We are ready immediately because all messages get queued anyway.
	(ready_handler_data->func)( app, ready_handler_data->data );

	while ( 1 ) {
		BOOL res = GetMessageW( &msg, 0, 0, 0);
		if ( res == 0 ) {
			exit_code = (int)msg.wParam;
			break;
		}
		else if (res == -1) {
			BW_WIN32_ASSERT_ERROR;
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
			/*else if ( msg.message == WM_APP + 1 ) {
				bw_WindowDispatchData* params = (bw_WindowDispatchData*)msg.lParam;

				(params->func)( params->window, params->data );

				free( params );
			}*/
		}
	}

	UnregisterClassW( L"bw-window", app->impl.handle );

	return exit_code;
}


bw_ApplicationImpl bw_ApplicationImpl_start( bw_Application* _app, int argc, char** argv ) {
	UNUSED(_app);
	UNUSED(argc);
	UNUSED(argv);

	bw_ApplicationImpl app;
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

int bw_Application_run( bw_Application* app, bw_ApplicationReadyFn on_ready, void* user_data ) {

}
