#include "cef.h"
#include "../assert.h"
#include "../application/cef.h"
#include "../browser_window.h"
#include "../cef/bw_handle_map.hpp"
#include "../cef/exception.hpp"
#include "../cef/util.hpp"
#include "../common.h"
#include "../debug.h"
#include "impl.h"

#include <string>
#include <include/base/cef_bind.h>
#include <include/cef_browser.h>
#include <include/cef_client.h>
#include <include/cef_v8.h>
#include <include/views/cef_browser_view.h>
#include <include/views/cef_window.h>

#ifdef BW_GTK
#include <gtk/gtk.h>
#include <gdk/gdkx.h>

// X11 headers
#ifdef CEF_X11
#include <X11/Xlib.h>
#endif
#endif

#ifdef BW_WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>

// QUICKFIX: The win32 definition of GetMessage is getting through from somewhere...
// TODO: Find out where
#undef GetMessage
#endif



void bw_BrowserWindowCef_connectToGtkWindow( bw_BrowserWindow* bw, CefWindowInfo& info, int width, int height );
void bw_BrowserWindowCef_connectToWin32Window( bw_BrowserWindow* bw, CefWindowInfo& info, int width, int height );



// Sends the given Javascript code to the renderer process, expecting the code to be executed over there.
// script_id should be a script id obtained from storing a callback in the eval callback store.
void bw_BrowserWindowCef_sendJsToRendererProcess(
	bw_BrowserWindow* bw,
	CefRefPtr<CefBrowser>& cef_browser,
	CefString& code,
	bw_BrowserWindowJsCallbackFn cb,
	void* user_data
);
char* bw_cef_errorMessage( bw_ErrCode code, const void* data );
/// Constructs the platform-specific window info needed by CEF.
CefWindowInfo _bw_BrowserWindow_windowInfo( bw_Window* window, int width, int height );



void bw_BrowserWindow_evalJs( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn cb, void* user_data ) {
	// Wrap the JS code within a temporary function and execute it, and convert the return value to a string
	// This allows executing JS code that isn't terminated with a semicolon, and does the javascript value string conversion inside JS.
	std::string _code = "(function () { return ";
	_code.append( js.data, js.len );
	_code += "; })()";
	CefString code = _code;
	// Note: For the sake of simplicity, I've used std::string to append some strings together.
	//       CefString unfortunately doesn't provide this functionality.
	//       There is some overhead because of this, but for now it is ok.

	// Execute the javascript on the renderer process, and invoke the callback from there:
	CefRefPtr<CefBrowser> cef_browser = *(CefRefPtr<CefBrowser>*)(bw->impl.cef_ptr);

	bw_BrowserWindowCef_sendJsToRendererProcess( bw, cef_browser, code, cb, user_data );
}

// It really doesn't matter from which thread we're sending the JavaScript code from,
//  we're sending it off to another process anyway.
void bw_BrowserWindow_evalJsThreaded( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn cb, void* user_data ) {
	bw_BrowserWindow_evalJs( bw, js, cb, user_data );
}

BOOL bw_BrowserWindow_getUrl(bw_BrowserWindow* bw, bw_StrSlice* url) {
	CefRefPtr<CefBrowser> cef_browser = *(CefRefPtr<CefBrowser>*)bw->impl.cef_ptr;

	CefString _url = cef_browser->GetMainFrame()->GetURL();
	*url = bw_cef_copyToStrSlice(_url);

	return TRUE;
}

#ifdef BW_GTK
void bw_BrowserWindowCef_connectToGtkWindow( bw_BrowserWindow* bw, CefWindowInfo& info, int width, int height ) {
#ifdef CEF_X11

	gtk_widget_show_all( bw->window->impl.handle );

	GdkWindow* gdk_window = gtk_widget_get_window( bw->window->impl.handle );
	Window x_window = GDK_WINDOW_XID( gdk_window );

	CefRect rect( 0, 0, width, height );
	
	info.SetAsChild( x_window, rect );
#else
#error BrowserWindow is set up to connect CEF with a GTK handle, but the glue for this is not yet implemented!
#endif
}
#endif

#ifdef BW_WIN32
void bw_BrowserWindowCef_connectToWin32Window( bw_BrowserWindow* bw, CefWindowInfo& info, int width, int height ) {

	RECT rect;
	GetClientRect((HWND)bw->window->impl.handle, &rect);
	CefRect crect(rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top);
	info.SetAsChild((HWND)bw->window->impl.handle, crect);
}
#endif

void bw_BrowserWindowCef_connectToWindow( bw_BrowserWindow* bw, CefWindowInfo& info, int width, int height ) {
	
#if defined(BW_WIN32)
	bw_BrowserWindowCef_connectToWin32Window( bw, info, width, height );
#elif defined(BW_GTK)
	bw_BrowserWindowCef_connectToGtkWindow( bw, info, width, height );
#elif defined(BW_CEF_WINDOW)
	// Nothing to do here...
#else
#error No supported windowing API implementation available
#endif
}

void bw_BrowserWindowImpl_clean(bw_BrowserWindowImpl* bw) {
	// Remove the link between our bw_BrowserWindow handle and the CefBrowser handle
	CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->cef_ptr;
	bw::bw_handle_map.drop( *cef_ptr );

	// Delete the CefBrowser pointer that we have stored in our bw_BrowserWindow handle
	delete cef_ptr;
	delete bw->resource_path;
}

bw_Err bw_BrowserWindow_navigate( bw_BrowserWindow* bw, bw_CStrSlice url ) {

	// TODO: Check if bw_CStrSlice can be converted into CefString in one step.
	std::string std_str( url.data, url.len );
	CefString cef_str( std_str );

	CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->impl.cef_ptr;
	(*cef_ptr)->GetMainFrame()->LoadURL( cef_str );

	BW_ERR_RETURN_SUCCESS;
}

void bw_BrowserWindowImpl_new(
	bw_BrowserWindow* browser,
	bw_BrowserWindowSource source,
	int width, int height,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,
	void* callback_data
) {
	CefWindowInfo info;
	CefBrowserSettings settings;

	// Set up a CefString with the source
	CefString source_string;
	if ( !source.is_html ) {
		std::string url = std::string( source.data.data, source.data.len );
		source_string = CefString( url );
	}
	else {
		std::string data = "data:text/html,";
		data.append( source.data.data, source.data.len );
		source_string = CefString( data );
	}

	// Update window size in CefWindowInfo
	bw_BrowserWindowCef_connectToWindow( browser, info, width, height );

	// Create the browser window handle
	bw_BrowserWindowImpl bw;
	bw.cef_ptr = 0;
	bw.resource_path = 0;
	
	// Store the resource path if set
	if ( browser_window_options->resource_path.len != 0 ) {
		bw.resource_path = new char[ browser_window_options->resource_path.len + 1 ];
		memcpy( bw.resource_path, browser_window_options->resource_path.data, browser_window_options->resource_path.len );
		bw.resource_path[ browser_window_options->resource_path.len ] = '\0';
	}

	// Create a CefDictionary containing the bw_BrowserWindow pointer to pass along CreateBrowser
	CefRefPtr<CefDictionaryValue> dict = CefDictionaryValue::Create();
	dict->SetBinary( "handle", CefBinaryValue::Create( (const void*)&browser, sizeof(browser) ) );
	dict->SetBinary( "callback", CefBinaryValue::Create( (const void*)&callback, sizeof(callback) ) );
	dict->SetBinary( "callback-data", CefBinaryValue::Create( (const void*)&callback_data, sizeof(callback_data) ) );
	dict->SetBool( "dev-tools", browser_window_options->dev_tools );
	
	// Create the browser
	CefRefPtr<CefClient>* cef_client = (CefRefPtr<CefClient>*)browser->window->app->engine_impl.cef_client;
#ifndef BW_CEF_WINDOW
	auto cef_browser = CefBrowserHost::CreateBrowserSync( info, *cef_client, source_string, settings, dict, nullptr );
#else
	// CefBrowserHost::CreateBrowser doesn't work well with Cef's own window layer, so we use the CefBrowserView
	CefRefPtr<CefBrowserView> browser_view = CefBrowserView::CreateBrowserView( *cef_client, source_string, settings, dict, nullptr, nullptr );
	CefRefPtr<CefWindow>* window = (CefRefPtr<CefWindow>*)browser->window->impl.handle_ptr;
	(*window)->AddChildView(browser_view);
	// Calling GetBrowser before AddChildView causes a segfault.
	auto cef_browser = browser_view->GetBrowser();
#endif

	bw::bw_handle_map.store(cef_browser, browser, callback, callback_data);
	CefRefPtr<CefBrowser>* cef_ptr = new CefRefPtr<CefBrowser>( cef_browser );
	bw.cef_ptr = (void*)cef_ptr;
	browser->impl = bw;

	if (browser_window_options->dev_tools)
#ifndef NDEBUG
		printf("Dev Tools are disabled for CEF in BrowserWindow, because it is broken. Please use remote debugging instead.\n");
#endif
}

void bw_BrowserWindowCef_sendJsToRendererProcess(
	bw_BrowserWindow* bw,
	CefRefPtr<CefBrowser>& cef_browser,
	CefString& code,
	bw_BrowserWindowJsCallbackFn cb,
	void* user_data
) {
	CefRefPtr<CefProcessMessage> msg = CefProcessMessage::Create("eval-js");
	CefRefPtr<CefListValue> args = msg->GetArgumentList();

	// eval-js message arguments
	args->SetString( 0, code );
	// Conver the callback data into binary blobs so we can send them to the renderer process
	CefRefPtr<CefBinaryValue> bw_bin = CefBinaryValue::Create( (const void*)&bw, sizeof( bw ) );
	args->SetBinary( 1, bw_bin );
	CefRefPtr<CefBinaryValue> cb_bin = CefBinaryValue::Create( (const void*)&cb, sizeof( cb ) );
	args->SetBinary( 2, cb_bin );
	CefRefPtr<CefBinaryValue> user_data_bin = CefBinaryValue::Create( (const void*)&user_data, sizeof( user_data ) );
	args->SetBinary( 3, user_data_bin );

	cef_browser->GetMainFrame()->SendProcessMessage( PID_RENDERER, msg );
}



void bw_BrowserWindowImpl_onResize( const bw_Window* window, unsigned int width, unsigned int height ) {
	bw_BrowserWindow* bw = (bw_BrowserWindow*)window->user_data;

	// Only do something when our browser window object and the underlying CEF implementation has been created.
	if ( bw != 0 && bw->impl.cef_ptr != 0 ) {

		CefRefPtr<CefBrowser> cef = *(CefRefPtr<CefBrowser>*)bw->impl.cef_ptr;

#if defined(BW_WIN32)
		SetWindowPos( cef->GetHost()->GetWindowHandle(), 0, 0, 0, width, height, SWP_SHOWWINDOW | SWP_NOZORDER | SWP_NOACTIVATE );
#elif defined(BW_GTK)
		//Window x_handle = cef->GetHost()->GetWindowHandle();

#endif
	}
}
