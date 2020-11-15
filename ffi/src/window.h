#ifndef BW_WINDOW_H
#define BW_WINDOW_H

#ifdef __cplusplus
extern "C" {
#endif

#include "application.h"
#include "string.h"

#if defined(BW_WIN32)
#include "win32/window.h"	// TODO: Make this "window/win32.h"
#elif defined(BW_GTK)
#include "window/gtk.h"
#else
#error Unsupported GUI implementation
#endif

#include <stdbool.h>



typedef struct bw_Window bw_Window;

typedef struct bw_WindowCallbacks {
	/// Fired just before the window gets destroyed and freed from memory.
	/// Should be implemented to free the user data provided to the window.
	void (*do_cleanup)( bw_Window* );
	/// Fired when the window has been closed, either by the user or programmatically.
	void (*on_close)( const bw_Window* );
	/// Fired when a window has finished loading
	void (*on_loaded)( const bw_Window* );
	/// Fired when a window is resizing
	void (*on_resize)( const bw_Window*, unsigned int width, unsigned int height );
} bw_WindowCallbacks;

typedef struct bw_WindowOptions {
	bool minimizable;
	bool resizable;
	bool closable;
	bool borders;
} bw_WindowOptions;

typedef void (*bw_WindowDispatchFn)( bw_Window* window, void* data );
typedef struct bw_WindowDispatchData bw_WindowDispatchData;



struct bw_Window {
	bw_Application* app;	// The application handle that this window belongs to.
	const bw_Window* parent;	// An optional window that acts as the parent to this window. If the parent gets destroyed, children will get destroyed too.
	bw_WindowImpl impl;	// The underlying handle to the window
	bool closed;	// Whether or not the window has been closed already
	bool dropped;	// Whether or not the window may be destroyed when it is actually closed
	bw_WindowCallbacks callbacks;
	void* user_data;
};



/// Creates a new (empty) window
/// The returned pointer is a handler for the window.
/// bw_Window_drop needs to be called on it after it is done being used,
///     otherwise the window is never actually destroyed and memory leakes happen.
bw_Window* bw_Window_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options,
	void* user_data
);

/// Hides the window from the user
void bw_Window_close( bw_Window* window );

/// Invalidates the window handle.
/// The window will get destroyed when it is deemed possible.
void bw_Window_drop( bw_Window* window );

/// Returns whether or not the window has been closed.
bool bw_Window_isClosed( const bw_Window* window );

/// Shows the window if it was previously hidden
void bw_Window_open( bw_Window* window );



#ifdef __cplusplus
}	// extern "C"
#endif

#endif//BW_WINDOW_H
