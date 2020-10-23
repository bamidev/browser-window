#ifndef BW_WINDOW_H
#define BW_WINDOW_H

#ifdef __cplusplus
extern "C" {
#endif

#include "application.h"
#include "string.h"
#ifdef BW_WIN32
#include "window.win32.h"
#else
#include "window/void.h"
#endif

#include <stdbool.h>



typedef struct bw_Window bw_Window;

typedef struct bw_WindowCallbacks {
	/// Fired just before the window gets destroyed and freed from memory.
	/// Should be implemented to free the user data provided to the window.
	void (*do_cleanup)( bw_Window* );
	/// Fired when the window has been closed, either by the user or programmatically.
	void (*on_close)( bw_Window* );
	/// Fired when the window is going to be destroyed.
	/// In that process, do_cleanup will be fired to force the implementor of the window to do its cleanup.
	void (*on_destroy)( bw_Window* );
	/// Fired when a window has finished loading
	void (*on_loaded)( bw_Window* );
} bw_WindowCallbacks;

typedef struct bw_WindowInner bw_WindowInner; // struct bw_WindowInner should already be declared.

typedef struct bw_WindowOptions {
	bool maximizable;
	bool minimizable;
	bool resizable;
	bool closable;
	bool borders;
	bool is_popup;
} bw_WindowOptions;

typedef void (*bw_WindowDispatchFn)( bw_Window* window, void* data );
typedef struct bw_WindowDispatchData bw_WindowDispatchData;



struct bw_Window {
	const bw_Application* app;	// The application handle that this window belongs to.
	const bw_Window* parent;	// An optional window that acts as the parent to this window. If the parent gets destroyed, children will get destroyed too.
	bw_WindowInner inner;	/// The underlying handle to the window
	bool closed;	// Whether or not the window has been closed by the user
	bw_WindowCallbacks callbacks;
	void* user_data;
};



/// Creates a new (empty) window
/// The returned pointer is a handler for the window.
/// bw_Window_drop needs to be called on it after it is done being used,
///     otherwise the window is never actually destroyed and memory leakes happen.
bw_Window* bw_Window_new(
	const bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options,
	void* user_data
);

/// Closes the window
/// Anything called for this window will still succeed after it is closed.
/// It will just not be visible anymore.
void bw_Window_close( bw_Window* window );

/// Should be called when the window is not needed anymore within the code, otherwise memory leaks happen.
/// This makes sure the window is destroyed when window closes, or already ís closed.
void bw_Window_drop( bw_Window* window );

/// Dispatches the given function on the GUI thread, and passes the given data along.
/// This function is thread-safe.
void bw_Window_dispatch( bw_Window* window, bw_WindowDispatchFn fn, void* data );

void _bw_Window_init( bw_Application* app );

/// Returns whether or not the window has been closed.
bool bw_Window_isClosed( const bw_Window* window );



#ifdef __cplusplus
}	// extern "C"
#endif

#endif//BW_WINDOW_H
