#include "cef.h"
#include "../assert.h"
#include "../application/cef.h"
#include "../browser_window.h"
#include "../cef/exception.hpp"

#include <string>
#include <include/base/cef_bind.h>
#include <include/cef_browser.h>
#include <include/cef_client.h>
#include <include/cef_v8.h>

// QUICKFIX: The win32 definition of GetMessage is getting through from some include
#undef GetMessage



/// Initialize the Cef's browser object
void bw_BrowserWindow_init_cef( CefRefPtr<CefBrowser> browser );
#ifdef BW_CEF_WINDOWS
RECT bw_BrowserWindow_window_rect( int width, int height );
#endif
char* bw_cef_error_message( bw_ErrCode code, const void* data );



// The task used to run javascript code in the renderer process
class bw_BrowserWindowEvalJsTask: public CefTask {
	bw_BrowserWindow* bw;
	CefString js;
	bw_BrowserWindowJsCallbackFn callback;
	void* user_data;

public:
	bw_BrowserWindowEvalJsTask( bw_BrowserWindow* bw, CefString js, bw_BrowserWindowJsCallbackFn callback, void* user_data ) :
		bw(bw), js(js), callback( callback ), user_data(user_data)	{}

	void Execute() override {

		CefString script_url( "eval" );
		CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->inner.cef_ptr;
		CefRefPtr<CefV8Value> ret_val;
		CefRefPtr<CefV8Exception> exception;

		bool result = (*cef_ptr)->GetMainFrame()->GetV8Context()->Eval( js, script_url, 0, ret_val, exception );
		if ( !result ) {
			bw_Err error = bw_cef_v8exc_to_bwerr( exception );
			callback( bw, user_data, NULL, &error );
		}
		else {
			std::string return_string = ret_val->GetStringValue().ToString();
			callback( bw, user_data, return_string.c_str(), NULL );
		}
	}

private:
	IMPLEMENT_REFCOUNTING( bw_BrowserWindowEvalJsTask );
};



void bw_BrowserWindow_close( bw_BrowserWindow* bw ) {
	// Actually close the brower
	CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->inner.cef_ptr;
	(*cef_ptr)->GetHost()->CloseBrowser( true );
}

void _bw_BrowserWindow_doCleanup( bw_BrowserWindow* ) {}

void bw_BrowserWindow_drop( bw_BrowserWindow* bw ) {
	CefRefPtr<CefBrowser>* cef_ptr = (CefRefPtr<CefBrowser>*)bw->inner.cef_ptr;
	delete cef_ptr;
	free( bw );
}

void bw_BrowserWindow_eval_js( bw_BrowserWindow* bw, bw_CStrSlice js, bw_BrowserWindowJsCallbackFn cb, void* user_data ) {

	// Wrap the JS code within a temporary function and execute it, converting the return value to a string
	std::string _code = "(function () { ";
	_code.append( js.data, js.len );
	_code += " })().toString()";
	CefString code = _code;

	// Execute the javascript on the renderer process, and invoke the callback from there:
	CefRefPtr<bw_BrowserWindowEvalJsTask> task( new bw_BrowserWindowEvalJsTask( bw, code, cb, user_data ) );
	CefPostTask( TID_RENDERER, task.get() );
}

void bw_BrowserWindow_init_cef( CefRefPtr<CefBrowser> browser ) {

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
	bw_BrowserWindowHandlerFn handler,	/// A function that gets invoked when javascript the appropriate call is made in javascript.
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
	bw->inner.cef_ptr = (void*)cef_ptr;

	bw_BrowserWindow_init_cef( *cef_ptr );

	return bw;
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
