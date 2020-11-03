#ifndef BW_APPLICATION_H
#define BW_APPLICATION_H



typedef void (*bw_ApplicationDispatchFn)( struct bw_Application* app, void* data );



#ifdef BW_WIN32
#include "application/win32.h"
#else
#error Unsupported platform
#endif

#ifdef __cplusplus
extern "C" {
#endif



typedef struct bw_Application bw_Application;
typedef struct bw_ApplicationEngineData bw_ApplicationEngineData;
typedef struct bw_ApplicationDispatchData bw_ApplicationDispatchData;

/// Initializes browser window.
/// Starts up browser engine process(es).
/// Returns an application handle.
bw_Application* bw_Application_start();

/// Exits the main loop, returning execution to the function that invoked the run call.
/// The exit_code will be returned by bw_Application_run.
void bw_Application_exit(  bw_Application* app, int exit_code );

/// Same as bw_Application_exit, but guaranteed to be thread-safe
/// The exit_code will be returned by bw_Application_run.
void bw_Application_exitAsync(  bw_Application* app, int exit_code );

/// Dispatches the given function to be executed on the thread this application instance has been created on,
///     and passes the given data to it.
/// This function is thread safe.
void bw_Application_dispatch( bw_Application* app, bw_ApplicationDispatchFn func, void* data );
void bw_Application_free( bw_Application* app );

/// Should be implemented by the source files implementing the browser engines.
/// Called by bw_Application_new.
int _bw_Application_init( bw_Application* app, int argc, const char* argv );

/// Should be implemented by the browser engine source files.
void bw_Application_init( bw_Application* app );

int bw_Application_run( bw_Application* app );

/// Should be implemented by the engine source files.
/// Can be used to perform work in the message loop.
void bw_Application_step();

/// Should be implemented by the source files implementing the browser engines.
/// Called by bw_Application_free.
void bw_Application_uninit( bw_Application* app );



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_APPLICATION_H
