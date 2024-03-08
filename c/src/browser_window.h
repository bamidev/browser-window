#ifndef BW_BROWSER_WINDOW_H
#define BW_BROWSER_WINDOW_H

#ifdef __cplusplus
extern "C" {
#endif

#ifdef BW_CEF
#include "browser_window/cef.h"
#elif defined(BW_EDGE2)
#include "browser_window/edge2.h"
#else
typedef struct {} bw_BrowserWindowImpl;
#endif

#include "application.h"
#include "event.h"
#include "err.h"
#include "string.h"
#include "window.h"

#include <stdint.h>



typedef struct bw_BrowserWindow bw_BrowserWindow;


typedef void (*bw_BrowserWindowCreationCallbackFn)( bw_BrowserWindow* window, void* data );
typedef void (*bw_BrowserWindowHandlerFn)( bw_BrowserWindow* window, bw_CStrSlice cmd, bw_CStrSlice* args, size_t arg_count );
typedef void (*bw_BrowserWindowJsCallbackFn)( bw_BrowserWindow* window, void* user_data, const char* result, const bw_Err* err );


typedef struct {
	bw_Event on_message;
	bw_Event on_page_title_changed;
	bw_Event on_navigation_start;
	bw_Event on_navigation_end;
	bw_Event on_tooltip;
} bw_BrowserWindowEvents;

/// `cmd` is always a string. The arguments are JS values in the form of a string.
typedef struct {
	bw_CStrSlice cmd;
	size_t arg_count;
	bw_CStrSlice* args;
} bw_BrowserWindowMessageArgs;

typedef struct bw_BrowserWindowOptions {
	BOOL dev_tools;
	bw_CStrSlice resource_path;
} bw_BrowserWindowOptions;

typedef struct bw_BrowserWindowSource {
	bw_CStrSlice data;
	BOOL is_html;
} bw_BrowserWindowSource;



struct bw_BrowserWindow {
	bw_Window* window;
	bw_BrowserWindowImpl impl;
	bw_BrowserWindowEvents events;
};


/// Executes the given JavaScript and calls the given callback (on the GUI thread) to provide the result.
void bw_BrowserWindow_evalJs( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn callback, void* cb_data );
void bw_BrowserWindow_evalJsThreaded( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn callback, void* cb_data );

void bw_BrowserWindow_free(bw_BrowserWindow* bw);

bw_Application* bw_BrowserWindow_getApp( bw_BrowserWindow* bw );
void* bw_BrowserWindow_getUserData( bw_BrowserWindow* bw );
BOOL bw_BrowserWindow_getUrl(bw_BrowserWindow* bw, bw_StrSlice* url);
bw_Window* bw_BrowserWindow_getWindow( bw_BrowserWindow* bw );

bw_Err bw_BrowserWindow_navigate( bw_BrowserWindow* bw, bw_CStrSlice url );

/// Allocates a browser window and creates the window for it.
/// Call `bw_BrowserWindow_create` on it to add the actual browser framework to this window.
bw_BrowserWindow* bw_BrowserWindow_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_CStrSlice title,
	int width, int height,
	const bw_WindowOptions* window_options
);

/// Adds the browser framework to the browser window.
/// Should be called right after `bw_BrowserWindow_new`.
void bw_BrowserWindow_create(
	bw_BrowserWindow* bw,
	int width, int height,
	bw_BrowserWindowSource source,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,	// A function that gets invoked when the browser window has been created.
	void* callback_data	// Data that will be passed to the creation callback
);


#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_BROWSER_WINDOW_H
