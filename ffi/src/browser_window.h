#ifndef BW_BROWSER_WINDOW_H
#define BW_BROWSER_WINDOW_H

#ifdef BW_EDGE
#include "browser_window/edge.h"
#endif
#ifdef BW_CEF
#include "browser_window/cef.h"
#endif




#include "application.h"
#include "err.h"
#include "string.h"
#include "window.h"



#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>



typedef struct bw_BrowserWindow bw_BrowserWindow;



typedef void (*bw_BrowserWindowCreationCallbackFn)( bw_BrowserWindow* window, void* data );
typedef void (*bw_BrowserWindowHandlerFn)( bw_BrowserWindow* window, bw_CStrSlice cmd, bw_CStrSlice* args, size_t arg_count );
typedef void (*bw_BrowserWindowJsCallbackFn)( bw_BrowserWindow* window, void* user_data, const char* result, const bw_Err* err );



typedef struct bw_BrowserWindowOptions {
	bool dev_tools;
} bw_BrowserWindowOptions;

typedef struct bw_BrowserWindowSource {
	bw_CStrSlice data;
	bool is_html;
} bw_BrowserWindowSource;



struct bw_BrowserWindow {
	bw_Window* window;
	bw_BrowserWindowImpl impl;
	bw_BrowserWindowHandlerFn external_handler;
	void* user_data;
};



void bw_BrowserWindow_close( bw_BrowserWindow* bw );

/// Marks the browser window handle as not being used anymore.
/// This makes it so that if and when the window gets closed, everything is freed and cleaned from memory.
/// This function can be called from any thread so it needs to be thread safe.
void bw_BrowserWindow_drop( bw_BrowserWindow* bw );

/// Executes the given JavaScript and calls the given callback (on the GUI thread) to provide the result.
void bw_BrowserWindow_evalJs( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn callback, void* cb_data );
void bw_BrowserWindow_evalJsThreaded( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn callback, void* cb_data );

bw_Application* bw_BrowserWindow_getApp( bw_BrowserWindow* bw );
void* bw_BrowserWindow_getUserData( bw_BrowserWindow* bw );

bw_Err bw_BrowserWindow_navigate( bw_BrowserWindow* bw, bw_CStrSlice url );

/// Creates a new browser window
void bw_BrowserWindow_new(
	bw_Application* app,
	const bw_BrowserWindow* parent,
	bw_BrowserWindowSource source,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* window_options,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowHandlerFn handler,	/// A function that gets invoked when javascript the appropriate call is made in javascript.
	void* user_data,	// The data that will be passed to the above handler function and the creation-callback when they are invoked.
	bw_BrowserWindowCreationCallbackFn callback,	// A function that gets invoked when the browser window has been created.
	void* callback_data	// Data that will be passed to the creation callback
);



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_H
