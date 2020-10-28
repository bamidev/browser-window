#include "cef.h"
#include "../assert.h"
#include "../application/cef.h"
#include "../browser_window.h"
#include "../cef/bw_handle_map.hpp"
#include "../cef/exception.hpp"

#include <string>
#include <include/base/cef_bind.h>
#include <include/cef_browser.h>
#include <include/cef_client.h>
#include <include/cef_v8.h>

// QUICKFIX: The win32 definition of GetMessage is getting through from somewhere...
// TODO: Find out where
#undef GetMessage



/// Initialize the Cef's browser object
void bw_BrowserWindow_init_cef( CefRefPtr<CefBrowser> browser );
#ifdef BW_CEF_WINDOWS
RECT bw_BrowserWindow_window_rect( int width, int height );
#endif
// Sends the given Javascript code to the renderer process, expecting the code to be executed over there.
// script_id should be a script id obtained from storing a callback in the eval callback store.
void bw_BrowserWindow_send_js_to_renderer_process( bw_BrowserWindow* bw, CefRefPtr<CefBrowser>& cef_browser, CefString& code, bw_BrowserWindowJsCallbackFn cb, void* user_data );
char* bw_cef_error_message( bw_ErrCode code, const void* data );



void bw_BrowserWindow_close( bw_BrowserWindow* bw ) {
	// Actually close the brower
	CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->inner.cef_ptr;
	(*cef_ptr)->GetHost()->CloseBrowser( true );
}

void _bw_BrowserWindow_doCleanup( bw_BrowserWindow* ) {}

void bw_BrowserWindow_drop( bw_BrowserWindow* bw ) {

	// Delete (a c++ feature) the CefBrowser pointer that we have allocated
	CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->inner.cef_ptr;
	bw::bw_handle_map.drop( *cef_ptr );
	delete cef_ptr;

	// Then free (a c feature) the browser window
	free( bw );
}

void bw_BrowserWindow_eval_js( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn cb, void* user_data ) {

	// Wrap the JS code within a temporary function and execute it, and convert the return value to a string
	// This allows executing JS code that isn't terminated with a semicolon, and does the javascript value string conversion inside JS.
	std::string _code = "(function () { return ";
	_code.append( js.data, js.len );
	_code += "; })().toString()";
	CefString code = _code;
	// Note: For the sake of simplicity, I've used std::string to append some strings together.
	//       CefString unfortunately doesn't provide this functionality.
	//       There is some overhead because of this, but for now it is ok.

	// Execute the javascript on the renderer process, and invoke the callback from there:
	CefRefPtr<CefBrowser> cef_browser = *(CefRefPtr<CefBrowser>*)(bw->inner.cef_ptr);

	bw_BrowserWindow_send_js_to_renderer_process( bw, cef_browser, code, cb, user_data );
}

void bw_BrowserWindow_init_cef( CefRefPtr<CefBrowser> browser ) {
	// We could do some initialization here...
}


bw_Err bw_BrowserWindow_navigate( bw_BrowserWindow* bw, bw_CStrSlice url ) {

	// TODO: Check if bw_CStrSlice can be converted into CefString in one step.
	std::string std_str( url.data, url.len );
	CefString cef_str( std_str );

	CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->inner.cef_ptr;
	(*cef_ptr)->GetMainFrame()->LoadURL( cef_str );

	BW_ERR_RETURN_SUCCESS;
}

bw_BrowserWindow* bw_BrowserWindow_new(
	const bw_Application* app,
	const bw_Window* parent,
	bw_BrowserWindowSource source,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* window_options,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowHandlerFn external_handler,	/// A function that gets invoked when javascript the appropriate call is made in javascript.
	void* user_data	/// The data that will be passed to the above handler function when it is invoked.
) {
	CefString title( std::string( _title.data, _title.len ) );

	CefWindowInfo info;
	CefBrowserSettings settings;
#ifdef BW_CEF_WINDOWS
	HWND parent_handle = HWND_DESKTOP;
#endif

	if ( parent != 0 ) {
		bw_BrowserWindow* parent_bw = (bw_BrowserWindow*)parent->user_data;
		CefRefPtr<CefBrowser>* parent_cef_ptr = (CefRefPtr<CefBrowser>*)(parent_bw->inner.cef_ptr);

		parent_handle = (*parent_cef_ptr)->GetHost()->GetWindowHandle();
	}

#ifdef BW_CEF_WINDOWS
	// On Windows, the following is required.
	// Possibly because CreateWindowEx doesn't work without a parent handle if it is not a popup.
	info.SetAsPopup( 0, title );

	RECT rect = bw_BrowserWindow_window_rect( width, height );

	//info.SetAsChild( parent_handle, bw_BrowserWindow_window_rect( width, height ) );
#endif

	// Set up a CefString with the source
	CefString source_string;
	if ( !source.is_html ) {
		source_string = CefString( std::string( source.data.data, source.data.len ) );
	}
	else {
		std::string data = "data:text/html,";
		data.append( source.data.data, source.data.len );
		source_string = CefString( data );
	}

	// TODO: Set title of window. If needed, through the win32 or other platform's methods...
	CefRefPtr<CefClient>* cef_client = (CefRefPtr<CefClient>*)app->cef_client;

	// Create the browser
	CefRefPtr<CefBrowser> _cef_ptr = CefBrowserHost::CreateBrowserSync( info, *cef_client, source_string, settings, NULL, NULL );
	CefRefPtr<CefBrowser>* cef_ptr = new CefRefPtr<CefBrowser>( _cef_ptr );
	auto bw = new bw_BrowserWindow;
	// Even though the CEF implementation doesn't use the window 'class', we still need to allocate it because some struct fields are still used.
	bw->window = bw_Window_new( app, parent, _title, width, height, window_options, user_data );
	bw->inner.cef_ptr = (void*)cef_ptr;
	bw->external_handler = external_handler;
	bw->user_data = user_data;
	//bw->callbacks <--- TODO

	// TODO: Apply width and height
	_cef_ptr->GetHost()->GetWindowHandle();

	// Store the cef browser handle with our handle in a global map
	bw::bw_handle_map.store( _cef_ptr, bw );

	bw_BrowserWindow_init_cef( *cef_ptr );

	return bw;
}

void bw_BrowserWindow_send_js_to_renderer_process( bw_BrowserWindow* bw, CefRefPtr<CefBrowser>& cef_browser, CefString& code, bw_BrowserWindowJsCallbackFn cb, void* user_data ) {
	CefRefPtr<CefProcessMessage> msg = CefProcessMessage::Create("eval-js");
	CefRefPtr<CefListValue> args = msg->GetArgumentList();

	// eval-js message arguments
	args->SetString( 0, code );
	CefRefPtr<CefBinaryValue> bw_bin = CefBinaryValue::Create( (const void*)&bw, sizeof( bw ) );
	args->SetBinary( 1, bw_bin );
	CefRefPtr<CefBinaryValue> cb_bin = CefBinaryValue::Create( (const void*)&cb, sizeof( cb ) );
	args->SetBinary( 2, cb_bin );
	CefRefPtr<CefBinaryValue> user_data_bin = CefBinaryValue::Create( (const void*)&user_data, sizeof( user_data ) );
	args->SetBinary( 3, user_data_bin );

	cef_browser->GetMainFrame()->SendProcessMessage( PID_RENDERER, msg );
}

#ifdef BW_CEF_WINDOWS
RECT bw_BrowserWindow_window_rect( int width, int height) {

	RECT desktop_rect;
	GetClientRect( GetDesktopWindow(), &desktop_rect );
	LONG desktop_width = desktop_rect.right - desktop_rect.left;
	LONG desktop_height = desktop_rect.bottom - desktop_rect.top;

	RECT rect;
	rect.left = -1;
	rect.top = -1;
	rect.right = -1;
	rect.bottom = -1;

	if ( width != -1 ) {
		rect.left = ( desktop_width - width ) / 2;
		rect.right = rect.left + width;
	}
	if ( height != -1 ) {
		rect.bottom = ( desktop_height - height ) / 2;
		rect.top = rect.bottom + height;
	}

	return rect;
}
#endif
