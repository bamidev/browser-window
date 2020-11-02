#include "../assert.h"
#include "../browser_window.h"
#include "../debug.h"
#include "../err.h"
#include "../win32.h"

#include <string>
#include <vector>
#include <winrt/Windows.Web.UI.Interop.h>


using namespace winrt::Windows::Foundation;
using namespace winrt::Windows::Web::UI::Interop;

struct HandleData {
	//WebViewProcess process;
	WebViewControl control;

	HandleData( const WebViewControl& ctrl ) : control(ctrl) {}
};



void bw_BrowserWindow_evalJs( bw_BrowserWindow* bw, bw_CStrSlice js_slice, bw_BrowserWindowJsCallbackFn callback, void* cb_data ) {
	auto hd = (HandleData*)bw->inner.webview;

	// Wrap the given javascript into a try/catch statement so that we can actually know if it returns an error or not
	std::string js = "(function(){try{return 'ok:'+(";
	js.append( js_slice.data, js_slice.len );
	js += ")}catch (e){return 'err:'+e.toString()}})()";
	winrt::hstring js_wstr = winrt::to_hstring( js.c_str() );

	auto op = hd->control.InvokeScriptAsync( L"eval",
		winrt::single_threaded_vector<winrt::hstring>(
			std::vector( 1, js_wstr )
		)
	);

	op.Completed([=](auto op, auto status) {

		if ( status == AsyncStatus::Error ) {
			bw_Err error;

			if ( op.ErrorCode() == 0x80020101 )
				error = bw_Err_new_with_msg( 1, "JavaScript syntax error" );
			else
				error = bw_win32_unhandledHresult( op.ErrorCode() );

			callback( bw, cb_data, 0, &error );

			bw_Err_free( &error );
		}
		else if ( status == AsyncStatus::Completed ) {
			std::string result = winrt::to_string( op.GetResults() );

			if ( result.find( "ok:" ) == 0 ) {
				auto result_str = result.substr(3);

				callback( bw, cb_data, result_str.c_str(), 0 );
			}
			else if ( result.find("err:") == 0 ) {
				std::string err_msg = "JavaScript error: ";
				err_msg += result.substr(4);
				bw_Err err = bw_Err_new_with_msg( 1, err_msg.c_str() );

				callback( bw, cb_data, 0, &err );

				bw_Err_free( &err );
			}
			else
				BW_ASSERT(0, "Invalid result received from Javascript!");
		}
	});
}

bw_Err bw_BrowserWindow_navigate( bw_BrowserWindow* bw, bw_CStrSlice url ) {
	auto hd = (HandleData*)bw->inner.webview;

	std::string url_std_str( url.data, url.len );
	Uri uri( winrt::to_hstring( url_std_str ) );

	hd->control.Navigate( uri );

	// Function Navigate doesn't provide any errors
	BW_ERR_RETURN_SUCCESS;
}

void bw_BrowserWindow_new(
	bw_Application* app,
	const bw_Window* parent,
	bw_BrowserWindowSource _source,
	bw_CStrSlice title,
	int width, int height,
	const bw_WindowOptions* window_options,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowHandlerFn external_handler,
	void* user_data,
	bw_BrowserWindowCreationCallbackFn callback,
	void* callback_data
) {
	// Create a copy of the source data because the data itself won't be available within the CreateWebViewControlAsync.Completed handler.
	std::string source( _source.data.data, _source.data.len );

	auto process = WebViewControlProcess();

	bw_Window* window = bw_Window_new( app, parent, title, width, height, window_options, 0 );

	auto op = process.CreateWebViewControlAsync( reinterpret_cast<long>(window->handle), Rect( 0.0, 0.0, (float)width, (float)height ) );

	op.Completed([=](auto op, auto status) {

		if ( status == AsyncStatus::Error )
			BW_WIN32_ASSERT_HRESULT( op.ErrorCode() );

		if ( status == AsyncStatus::Completed ) {
			HandleData* handle_data = new HandleData( op.GetResults() );

			// Construct browser window handle
			bw_BrowserWindow* bw = new bw_BrowserWindow;
			bw->window = window;
			bw->inner.webview = (void*)handle_data;
			bw->external_handler = external_handler;
			bw->user_data = user_data;
			window->user_data = (void*)bw;	// Store a pointer of our browser window into the window
			_bw_BrowserWindow_initWindowCallbacks( bw );

			// Allow calling back to us from js
			handle_data->control.Settings().IsScriptNotifyAllowed( true );
			// Is needed in order to be visible to the user
			handle_data->control.IsVisible( true );

			handle_data->control.ScriptNotify([=](auto, auto const& args) {BW_DEBUG("on SCRIPTNOTIFY")
				std::string args_str = winrt::to_string( args.Value() );

				BW_DEBUG("ScriptNotify %s", args_str.c_str())
			});

			// Inject javascript that creates the invoke_extern function
			handle_data->control.NavigationStarting([=](auto const& sender, auto const& args) {
				handle_data->control.AddInitializeScript(winrt::to_hstring(
					"function() invoke_extern( cmd ) { window.external.notify( cmd ) }"
				));
			});

			// Navigate to the given source

			auto source_str = winrt::to_hstring( source );
			if ( _source.is_html ) {

				handle_data->control.NavigateToString( source_str );
			}
			else {

				Uri uri( source_str );

				handle_data->control.Navigate( uri );
			}

			// Invoke callback
			callback( bw, callback_data );
		}
	});
}
