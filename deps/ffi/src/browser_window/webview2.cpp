/// WebView2 is corresponds to the Microsoft Egde browser engine that actually makes use of the chromium browser engine.
#include "../browser_window.h"

#include <wrl.h>
#include <wil/com.h>
#include <WebView2.h>

#include "../win32.h"
#include "../window.h"

#define DEFAULT_EDGE_LOCATION L"C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\84.0.522.52"

#define ASSERT_ON_FAIL( HR_STATEMENT ) \
	{ \
		HRESULT r = (HR_STATEMENT); \
		if ( r != 0 ) { \
			BW_WIN32_ASSERT_HRESULT( r ); \
		} \
	}



#pragma comment(lib, "WebView2Loader.dll.lib")



using namespace Microsoft::WRL;



void _bw_BrowserWindow_doCleanup( bw_BrowserWindow* bw ) {

}

void bw_BrowserWindow_eval_js( bw_BrowserWindow* bw, bw_CStrSlice _js, bw_BrowserWindowJsCallbackFn callback, void* cb_data ) {
	WCHAR* js = bw_win32_copyAsNewWstr( _js );

	reinterpret_cast<ICoreWebView2*>(bw->inner.webview)->ExecuteScript( js, Callback<ICoreWebView2ExecuteScriptCompletedHandler>(
		[bw, cb_data, callback]( HRESULT error_code, LPCWSTR result ) -> HRESULT {

			if ( error_code != 0 ) {
				bw_Err err = bw_win32_unhandledHresult( error_code );
				callback( bw, cb_data, 0, &err );
			}
			else {
				char* cstr = bw_win32_copyWstrAsNewCstr( result );
				callback( bw, cb_data, cstr, 0 );
				free( cstr );
			}

			return S_OK;
		}).Get()
	);
}

bw_Err bw_BrowserWindow_navigate( bw_BrowserWindow* bw, bw_CStrSlice _url ) {
	WCHAR* url = bw_win32_copyAsNewWstr( _url );

	HRESULT res = reinterpret_cast<ICoreWebView2*>(bw->inner.webview)->Navigate( url );
	if ( res != 0 )
		return bw_win32_unhandledHresult( res );

	SysFreeString( url );

	BW_ERR_RETURN_SUCCESS;
}

/// Creates a new browser window without any content
bw_BrowserWindow* bw_BrowserWindow_new(
	const bw_Application* app,
	const bw_Window* parent,
	bw_BrowserWindowSource source,
	bw_CStrSlice title,
	int width, int height,
	const bw_WindowOptions* window_options,
	const bw_BrowserWindowOptions* _options,
	bw_BrowserWindowHandlerFn handler,
	void* user_data
) {
	// The options are passed to callbacks that might run after the passed options live.
	// So we copy them just to be sure...
	bw_BrowserWindowOptions options = *_options;

	bw_BrowserWindow* browser_window = new bw_BrowserWindow;
	browser_window->window = bw_Window_new( app, parent, title, width, height, window_options, browser_window );
	browser_window->handler = handler;
	browser_window->user_data = user_data;
	_bw_BrowserWindow_init_window_callbacks( browser_window );

	// The pointer of source.data may not be valid anymore when a callback is fired.
	WCHAR* source_data = bw_win32_copyAsNewWstr( source.data );

	// TODO: Instead of using the default location that edge is installed in, look up the install dir from the registry, and then default back to the default install dir
	HRESULT result = CreateCoreWebView2EnvironmentWithOptions( nullptr, nullptr, nullptr,

		Callback<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>(
			[browser_window, options, source, source_data](HRESULT result, ICoreWebView2Environment* env) -> HRESULT {

				// Create a CoreWebView2Controller and get the associated CoreWebView2 whose parent is the main window hWnd
				HRESULT r = env->CreateCoreWebView2Controller( browser_window->window->inner.handle, Callback<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>(
					[browser_window, options, source, source_data](HRESULT result, ICoreWebView2Controller* controller) -> HRESULT {
						if (controller != nullptr) {

							browser_window->inner.webview_controller = controller;

							ASSERT_ON_FAIL( controller->get_CoreWebView2( reinterpret_cast<ICoreWebView2**>(&browser_window->inner.webview) ) );

							auto webview = reinterpret_cast<ICoreWebView2*>( browser_window->inner.webview );

							// Add a few settings for the webview
							// The demo step is redundant since the values are the default settings
							ICoreWebView2Settings* settings;
							ASSERT_ON_FAIL( webview->get_Settings(&settings) );

							ASSERT_ON_FAIL( settings->put_IsScriptEnabled(true) );
							ASSERT_ON_FAIL( settings->put_AreDefaultScriptDialogsEnabled(true) );
							ASSERT_ON_FAIL( settings->put_IsWebMessageEnabled(true) );
							ASSERT_ON_FAIL( settings->put_AreDevToolsEnabled(options.dev_tools) );

							// Resize WebView to fit the bounds of the parent window
							RECT bounds;
							GetClientRect(browser_window->window->inner.handle, &bounds);
							ASSERT_ON_FAIL( controller->put_Bounds(bounds) );

							// Navigate to the source provided
							// If it is an URL:

							if ( !source.is_html ) {
								ASSERT_ON_FAIL( webview->Navigate( source_data ) );
							}
							// If it is plain HTML:
							else {
								ASSERT_ON_FAIL( webview->NavigateToString( source_data ) );
							}
							free( source_data );


							// Fire on_laoded callback
							//if ( browser_window->callbacks.on_loaded != 0 )
							//	browser_window->callbacks.on_loaded( browser_window );

							if ( !UpdateWindow( browser_window->window->inner.handle ) ) {
								BW_WIN32_ASSERT_ERROR;
							}
							fprintf(stderr,"Done!\n");

							return S_OK;
						}
					}
				).Get() );

				if ( r != 0 ) {
					BW_WIN32_ASSERT_HRESULT( r );
				}

				return S_OK;
			}
		).Get()
	);

	if ( result != 0 ) {

		if ( result == __HRESULT_FROM_WIN32( ERROR_FILE_NOT_FOUND ) ) {
			fprintf( stderr, "Microsoft Edge (chromium) installation not found!\n" );
			assert(0);
		}
		else {
			BW_WIN32_ASSERT_HRESULT( result );
		}
	}

	return browser_window;
}
