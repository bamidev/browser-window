#ifndef BW_APPLICATION_H
#define BW_APPLICATION_H



typedef void (*bw_ApplicationDispatchFn)( struct bw_Application* app, void* data );



#ifdef BW_WIN32
#include "application/win32.h"
#else
#ifdef BW_CEF
#include "application/cef.h"
#else
#error Unsupported platform
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif



typedef struct bw_Application bw_Application;
typedef struct bw_ApplicationDispatchData bw_ApplicationDispatchData;

/// Creates a new application instance
bw_Application* bw_Application_new();

/// Exits the main loop, returning execution to the function that invoked the run call.
/// The exit_code will be returned by bw_Application_run.
void bw_Application_exit(  bw_Application* app, int exit_code );

/// Same as bw_Application_exit, but guaranteed to be thread-safe
/// The exit_code will be returned by bw_Application_run.
void bw_Application_exit_async(  bw_Application* app, int exit_code );

/// Dispatches the given function to be executed on the thread this application instance has been created on,
///     and passes the given data to it.
/// This function is thread safe.
extern void bw_Application_dispatch( bw_Application* app, bw_ApplicationDispatchFn func, void* data );
void bw_Application_free( bw_Application* app );

/// Should be implemented by the source files implementing the browser engines.
/// Called by bw_Application_new.
int _bw_Application_init( bw_Application* app, int argc, const char* argv );

int bw_Application_run( bw_Application* app );

/// Should be implemented by the source files implementing the browser engines.
/// Called by bw_Application_free.
void _bw_Application_uninit( bw_Application* app );



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_APPLICATION_H
