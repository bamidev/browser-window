#ifndef BW_APPLICATION_H
#define BW_APPLICATION_H

#ifdef __cplusplus
extern "C" {
#endif



struct bw_Application;
typedef void (*bw_ApplicationDispatchFn)( struct bw_Application* app, void* data );
typedef bw_ApplicationDispatchFn bw_ApplicationReadyFn;



// Import bw_ApplicationImpl and bw_ApplicationEngineImpl definitions
#if defined(BW_WIN32)
#include "application/win32.h"
#elif defined(BW_GTK)
#include "application/gtk.h"
#else
#error Unsupported platform
#endif
#if defined(BW_CEF)
#include "application/cef.h"
#elif defined(BW_EDGE)
#include "application/edge.h"
#else
#error Unsupported engine
#endif


struct bw_Application {
	bw_ApplicationImpl impl;
	bw_ApplicationEngineImpl engine_impl;	/// Can be set by the implementation of a browser engine
	unsigned int windows_alive;
};

typedef struct bw_Application bw_Application;
typedef struct bw_ApplicationEngineData bw_ApplicationEngineData;
typedef struct bw_ApplicationDispatchData bw_ApplicationDispatchData;



/// Safety check that makes sure the given application handle is used on the correct thread.
/// Does nothing in release mode.
void bw_Application_checkThread( const bw_Application* );

/// Initializes browser window.
/// Starts up browser engine process(es).
/// Returns an application handle.
bw_Application* bw_Application_start( int argc, char** argv );

/// Exits the main loop, returning execution to the function that invoked the run call.
/// The exit_code will be returned by bw_Application_run.
void bw_Application_exit(  bw_Application* app, int exit_code );

/// Same as bw_Application_exit, but guaranteed to be thread-safe
/// The exit_code will be returned by bw_Application_run.
void bw_Application_exitAsync(  bw_Application* app, int exit_code );

/// Dispatches the given function to be executed on the thread this application instance has been created on,
///     and passes the given data to it.
/// This function is thread safe.
///
/// # Returns
/// An indication of whether or not the function was able to be dispatched.
/// Dispatching a function fails when the application has already been terminated.
bool bw_Application_dispatch( bw_Application* app, bw_ApplicationDispatchFn func, void* data );

/// Should be called on the application handle at the end of the program.
/// This invalidates the handle.
void bw_Application_finish( bw_Application* app );

bool bw_Application_isRunning( const bw_Application* app );

/// Runs the event loop.
/// Calls the `on_ready` callback when `app` can be used.
int bw_Application_run( bw_Application* app, bw_ApplicationReadyFn on_ready, void* user_data );



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_APPLICATION_H
