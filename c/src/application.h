#ifndef BW_APPLICATION_H
#define BW_APPLICATION_H

#ifdef __cplusplus
extern "C" {
#endif

#include "bool.h"
#include "string.h"



struct bw_Application;
typedef void (*bw_ApplicationDispatchFn)( struct bw_Application* app, void* data );
typedef bw_ApplicationDispatchFn bw_ApplicationReadyFn;



#ifndef BW_BINDGEN

// Import bw_ApplicationImpl and bw_ApplicationEngineImpl definitions
#if defined(BW_WIN32)
#include "application/win32.h"
#elif defined(BW_GTK)
#include "application/gtk.h"
#elif defined(BW_CEF_WINDOW)
#include "application/cef_window.h"
#else
#error Unsupported platform
#endif

#if defined(BW_CEF)
#include "application/cef.h"
#else
typedef struct {} bw_ApplicationEngineImpl;
#endif

#else
typedef struct {} bw_ApplicationImpl;
typedef struct {} bw_ApplicationEngineImpl;
#endif

#include "err.h"



struct bw_Application {
	unsigned int windows_alive;
	BOOL finished;
	bw_ApplicationImpl impl;
	bw_ApplicationEngineImpl engine_impl;	/// Can be set by the implementation of a browser engine
};

typedef struct bw_Application bw_Application;
typedef struct bw_ApplicationEngineData bw_ApplicationEngineData;

typedef struct {
	bw_ApplicationDispatchFn func;
	void* data;
} bw_ApplicationDispatchData;

typedef struct {
	bw_CStrSlice engine_seperate_executable_path;
	bw_CStrSlice resource_dir;
} bw_ApplicationSettings;



/// Safety check that makes sure the given application handle is used on the correct thread.
/// Does nothing in release mode.
void bw_Application_assertCorrectThread( const bw_Application* );

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
BOOL bw_Application_dispatch( bw_Application* app, bw_ApplicationDispatchFn func, void* data );

/// Shuts down all application processes and performs necessary clean-up code.
void bw_Application_finish( bw_Application* app );

/// Frees memory for the application handle.
/// Call `bw_Application_finish` before you call this function.
/// Freeing the application handle is generally not necessary, as all memory in use by the process gets released anyway after shutdown.
void bw_Application_free( bw_Application* app );

/// Initializes browser window.
/// Starts up browser engine process(es).
/// Returns an application handle.
bw_Err bw_Application_initialize( bw_Application** application, int argc, char** argv, const bw_ApplicationSettings* settings );

BOOL bw_Application_isRunning( const bw_Application* app );

/// Runs the event loop.
/// Calls the `on_ready` callback when `app` can be used.
int bw_Application_run( bw_Application* app, bw_ApplicationReadyFn on_ready, void* user_data );



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_APPLICATION_H
