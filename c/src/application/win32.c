#pragma comment(lib, "user32.lib")
#pragma comment(lib, "gdi32.lib")
#pragma comment(lib, "ole32.lib")

#include "../assert.h"
#include "../application.h"
#include "../common.h"

#include "impl.h"

#include <stdlib.h>
#define WIN32_LEAN_AND_MEAN
#include <windef.h>
#include <windows.h>

#include "../win32.h"
#include "../window/win32.h"

#include <stdio.h>


typedef struct {
	void* next;
	UINT_PTR timer_id;
	bw_Application* app;
	bw_ApplicationDispatchData* dispatch_data;
} bw_ApplicationTimerMapEntry;



void bw_ApplicationWin32_dispatchWrapper(bw_Application* app, void* _data);
void bw_ApplicationWin32_setTimer(bw_Application* app, bw_ApplicationDispatchData* dispatch_data, uint64_t delay);
void bw_ApplicationWin32_timerHandler(HWND _hwnd, UINT _, UINT_PTR nIDEvent, DWORD _2);



bw_ApplicationTimerMapEntry* timer_map = NULL;


void bw_Application_assertCorrectThread( const bw_Application* app ) {
#ifndef NDEBUG
	if ( app->impl.handle != GetModuleHandle(NULL) )
		BW_PANIC("Application handle used in invalid thread!");
#else
	UNUSED(app);
#endif
}

BOOL bw_ApplicationImpl_dispatch( bw_Application* app, bw_ApplicationDispatchData* dispatch_data ) {
	
	// Check if the runtime is still running
	AcquireSRWLockShared( &app->impl.is_running_mtx );
	bool result = app->is_running;
	ReleaseSRWLockShared( &app->impl.is_running_mtx );
	if ( result == FALSE )
		return FALSE;

	PostThreadMessageW( app->impl.thread_id, WM_APP, (WPARAM)NULL, (LPARAM)dispatch_data );

	return TRUE;
}

BOOL bw_ApplicationImpl_dispatchDelayed(bw_Application* app, bw_ApplicationDispatchData* dispatch_data,  uint64_t milliseconds) {
	BW_ASSERT(milliseconds < (1ul << (8*sizeof(UINT)-1)), "too many milliseconds");
	// TODO: Support up to 2^62-1 milliseconds using the timer multiple times.

	// Check if the runtime is still running
	AcquireSRWLockShared( &app->impl.is_running_mtx );
	bool result = app->is_running;
	ReleaseSRWLockShared( &app->impl.is_running_mtx );
	if ( result == FALSE )
		return FALSE;

	// If we are on the right thread, just call `SendMessageTimeout`.
	if (app->impl.thread_id == GetCurrentThreadId()) {
		bw_ApplicationWin32_setTimer(app, dispatch_data, milliseconds);
	}
	// Otherwise, post to the GUI thread first
	else {
		bw_ApplicationDispatchDelayedData* delayed_data = (bw_ApplicationDispatchDelayedData*)malloc(sizeof(bw_ApplicationDispatchDelayedData));
		delayed_data->dispatch_data = dispatch_data;
		delayed_data->delay = (UINT)milliseconds;
		delayed_data->app = app;

		bw_ApplicationDispatchData* wrapper_data = (bw_ApplicationDispatchData*)malloc(sizeof(bw_ApplicationDispatchData));
		wrapper_data->func = bw_ApplicationWin32_dispatchWrapper;
		wrapper_data->data = (void*)delayed_data;

		bw_ApplicationImpl_dispatch(app, wrapper_data);
	}

	return TRUE;
}

void bw_ApplicationWin32_addToTimerMap(bw_Application* app, UINT_PTR timer_id, bw_ApplicationDispatchData* dispatch_data) {

	// Create new timer map entry
	bw_ApplicationTimerMapEntry* new_entry = (bw_ApplicationTimerMapEntry*)malloc(sizeof(bw_ApplicationTimerMapEntry));
	new_entry->next = 0;
	new_entry->timer_id = timer_id;
	new_entry->app = app;
	new_entry->dispatch_data = dispatch_data;

	if (timer_map != 0) {
		bw_ApplicationTimerMapEntry* entry = timer_map;

		// Skip all but last entry
		while (entry->next != 0) {
			entry = (bw_ApplicationTimerMapEntry*)entry->next;
		}

		entry->next = new_entry;
	}
	else {
		timer_map = new_entry;
	}
}

void bw_ApplicationWin32_dispatchWrapper(bw_Application* app, void* _data) {
	bw_ApplicationDispatchDelayedData* data = (bw_ApplicationDispatchDelayedData*)_data;
	
	bw_ApplicationWin32_setTimer(app, (bw_ApplicationDispatchData*)data->dispatch_data, data->delay);

	free(data);
}

bw_ApplicationTimerMapEntry* bw_ApplicationWin32_findInTimerMap(UINT_PTR timer_id) {
	if (timer_map == 0)
		return 0;

	bw_ApplicationTimerMapEntry* entry = timer_map;
	while (entry->timer_id != timer_id) {
		if (entry->next == 0)
			return 0;
	}

	return entry;
}

void bw_ApplicationWin32_freeTimerMap() {
	bw_ApplicationTimerMapEntry* entry = timer_map;

	while (entry != 0) {
		bw_ApplicationTimerMapEntry* next = (bw_ApplicationTimerMapEntry*)entry->next;

		KillTimer(NULL, entry->timer_id);
		free(entry);

		entry = next;
	}
}

bool bw_ApplicationWin32_removeFromTimerMap(bw_ApplicationTimerMapEntry* entry) {
	
	if (timer_map == entry) {
		timer_map = (bw_ApplicationTimerMapEntry*)entry->next;
		free(entry);
		return true;
	}

	bw_ApplicationTimerMapEntry* prev = timer_map;
	bw_ApplicationTimerMapEntry* i = (bw_ApplicationTimerMapEntry*)prev->next;
	while (i != 0) {
		if (i == entry) {
			prev = (bw_ApplicationTimerMapEntry*)i->next;
			free(i);
			return true;
		}

		i = (bw_ApplicationTimerMapEntry*)i->next;
	}

	return false;
}

void bw_ApplicationWin32_setTimer(bw_Application* app, bw_ApplicationDispatchData* dispatch_data, uint64_t delay) {
	UINT_PTR timer_id = SetTimer(0, 0, delay, (TIMERPROC)bw_ApplicationWin32_timerHandler);

	bw_ApplicationWin32_addToTimerMap(app, timer_id, dispatch_data);
}

void bw_ApplicationWin32_timerHandler(HWND hwnd, UINT _, UINT_PTR timer_id, DWORD _2) {
	UNUSED(_);
	UNUSED(_2);
	KillTimer(hwnd, timer_id);

	bw_ApplicationTimerMapEntry* entry = bw_ApplicationWin32_findInTimerMap(timer_id);

	bw_ApplicationDispatchData* dispatch_data = (bw_ApplicationDispatchData*)entry->dispatch_data;

	dispatch_data->func(entry->app, dispatch_data->data);
	
	free(dispatch_data);
	bw_ApplicationWin32_removeFromTimerMap(entry);

}



int bw_ApplicationImpl_run( bw_Application* app, bw_ApplicationImpl_ReadyHandlerData* ready_handler_data ) {

	MSG msg;
	BOOL res;
	int exit_code = 0;

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
	return exit_code;
}


bw_ApplicationImpl bw_ApplicationImpl_initialize( bw_Application* _app, int argc, char** argv, const bw_ApplicationSettings* settings ) {
	UNUSED(_app);
	UNUSED(argc);
	UNUSED(argv);
	UNUSED(settings);

	bw_ApplicationImpl app;
	InitializeSRWLock( &app.is_running_mtx );
	app.thread_id = GetCurrentThreadId();
	app.handle = GetModuleHandle(NULL);

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
	bw_ApplicationWin32_freeTimerMap();
	UnregisterClassW( L"bw-window", app->handle );
}
