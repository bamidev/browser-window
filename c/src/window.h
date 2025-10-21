#ifndef BW_WINDOW_H
#define BW_WINDOW_H

#ifdef __cplusplus
extern "C" {
#endif

#include "application.h"
#include "common.h"
#include "string.h"

#include <stdbool.h>
#include <stdint.h>



typedef struct bw_Window bw_Window;

typedef struct bw_WindowOptions {
	bool borders;
	bool minimizable;
	bool resizable;
} bw_WindowOptions;

typedef void (*bw_WindowDispatchFn)( bw_Window* window, void* data );
typedef struct bw_WindowDispatchData bw_WindowDispatchData;


#if defined(BW_WIN32)
#include "window/win32.h"
#elif defined(BW_GTK)
#include "window/gtk.h"
#elif defined(BW_CEF_WINDOW)
#include "window/cef.h"
#else
typedef struct {} bw_WindowImpl;
#endif


typedef struct bw_BrowserWindow bw_BrowserWindow;

struct bw_Window {
	bw_Application* app;	// The application handle that this window belongs to.
	const bw_Window* parent;	// An optional window that acts as the parent to this window. If the parent gets destroyed, children will get destroyed too.
	bool closed;	// Whether or not the window has been closed already
	bool dropped;	// Whether or not the window may be destroyed when it is actually closed
	void* user_data;	// TODO: Put the user_data ptr on bw_BrowserWindow
	bw_BrowserWindow* browser;
	bw_WindowImpl impl;	// Data for the implementation of the window
};


/// Hides the window and frees the user data
void bw_Window_close(bw_Window* window);

/// Invalidates the window handle.
void bw_Window_free(bw_Window* window);

/// Frees the user data that was attached to this window.
void bw_Window_freeUserData(bw_Window* window);

/// Gets the width and height of the usable area inside the window.
bw_Dims2D bw_Window_getContentDimensions( bw_Window* window );

/// Gets the opacity of the window as a value from 0 to 255.
uint8_t bw_Window_getOpacity( bw_Window* window );

/// Gets the X and Y coordinates of the window position relative to the desktop screen.
bw_Pos2D bw_Window_getPosition( bw_Window* window );

/// Copies as many bytes into `title` that fit in there.
/// Returns the number of characters the title actually has.
size_t bw_Window_getTitle( bw_Window* window, char** title );

/// Gets the width and height of the window including the title bar and borders.
bw_Dims2D bw_Window_getWindowDimensions( bw_Window* window );

// Makes the window invisible and inaccessible from the user.
void bw_Window_hide( bw_Window* window );

void* bw_Window_innerHandle(bw_Window* window);

/// Returns whether or not the window is not hidden.
/// `bw_Window_show` and `bw_Window_hide` change the visibility.
bool bw_Window_isVisible( const bw_Window* window );

/// Creates a new (empty) window
/// The returned pointer is a handler for the window.
/// bw_Window_drop needs to be called on it after it is done being used,
///     otherwise the window is never actually destroyed and memory leakes happen.
bw_Window* bw_Window_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options
);


void bw_Window_setContentDimensions( bw_Window* window, bw_Dims2D dimensions );

void bw_Window_setOpacity( bw_Window* window, uint8_t opacity );

void bw_Window_setPosition( bw_Window* window, bw_Pos2D position );

void bw_Window_setUserData(bw_Window* bw, void* user_data);

/// Applies the given title;
void bw_Window_setTitle( bw_Window* window, bw_CStrSlice title );

void bw_Window_setWindowDimensions( bw_Window* window, bw_Dims2D dimensions );

/// Shows the window if it was previously hidden
/// Is generally called after window creation.
void bw_Window_show( bw_Window* window );

void bw_Window_triggerClose( bw_Window* window );



void _bw_Window_onResize( const bw_Window* window, unsigned int width, unsigned int height );



#ifdef __cplusplus
}	// extern "C"
#endif

#endif//BW_WINDOW_H
