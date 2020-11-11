#include "../application.h"
#include "../assert.h"
#include "../browser_window.h"
#include "../debug.h"
#include "../err.h"
#include "../win32.h"
#include "impl.h"

#include <sstream>
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

struct bw_BrowserWindowEdge_EvalJsCallbackData {
	bw_BrowserWindow* bw;
	bw_CStrSlice js_slice;
	bw_BrowserWindowJsCallbackFn callback;
	void* cb_data;
};



// Handles the resizing of the browser
void bw_BrowserWindowImpl_onResize( const bw_Window* window, unsigned int width, unsigned int height );



std::vector<std::string> parse_args( const char* args_string ) {
	std::string tmp;
	std::vector<std::string> stk;
	std::stringstream ss(args_string);
	while( std::getline( ss, tmp, '\x03' ) ) {
		stk.push_back( tmp );
	}
	return stk;
}

std::vector<bw_CStrSlice> args_slices( const std::vector<std::string>& args ) {

	std::vector<bw_CStrSlice> vec; vec.reserve( args.size() );

	for ( auto it = args.begin(); it != args.end(); it++ ) {
		const std::string& arg = *it;

		bw_CStrSlice slice = { arg.length(), arg.c_str() };
		vec.push_back( slice );
	}

	return vec;
}

void bw_BrowserWindow_evalJs( bw_BrowserWindow* bw, bw_CStrSlice js_slice, bw_BrowserWindowJsCallbackFn callback, void* cb_data ) {
	auto hd = (HandleData*)bw->impl.data;

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

		// on error
		if ( status == AsyncStatus::Error ) {
			bw_Err error;

			if ( op.ErrorCode() == 0x80020101 )
				error = bw_Err_new_with_msg( 1, "JavaScript syntax error" );
			else
				error = bw_win32_unhandledHresult( op.ErrorCode() );

			callback( bw, cb_data, 0, &error );

			bw_Err_free( &error );
		}
		// on success
		else if ( status == AsyncStatus::Completed ) {
			std::string result = winrt::to_string( op.GetResults() );

			// If result indicates success
			if ( result.find( "ok:" ) == 0 ) {
				auto result_str = result.substr(3);

				callback( bw, cb_data, result_str.c_str(), 0 );
			}
			// If result indicates error
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

void bw_BrowserWindowEdge_evalJsHandler( bw_Application* app, void* _data ) {
	auto data = (bw_BrowserWindowEdge_EvalJsCallbackData*)_data;

	bw_BrowserWindow_evalJs( data->bw, data->js_slice, data->callback, data->cb_data );

	delete data;
}

void bw_BrowserWindow_evalJsThreaded( bw_BrowserWindow* bw, bw_CStrSlice js_slice, bw_BrowserWindowJsCallbackFn callback, void* cb_data ) {

	bw_BrowserWindowEdge_EvalJsCallbackData* data = new bw_BrowserWindowEdge_EvalJsCallbackData;
	data->bw = bw;
	data->js_slice = js_slice;
	data->callback = callback;
	data->cb_data = cb_data;

	bw_Application_dispatch( bw->window->app, bw_BrowserWindowEdge_evalJsHandler, (void*)data );
}

void bw_BrowserWindowImpl_doCleanup( bw_Window* window ) {

	auto bw = (bw_BrowserWindow*)window->user_data;
	auto hd = (HandleData*)bw->impl.data;
	delete hd;
}

bw_Err bw_BrowserWindow_navigate( bw_BrowserWindow* bw, bw_CStrSlice url ) {
	auto hd = (HandleData*)bw->impl.data;

	std::string url_std_str( url.data, url.len );
	Uri uri( winrt::to_hstring( url_std_str ) );

	hd->control.Navigate( uri );

	// Function Navigate doesn't provide any errors
	BW_ERR_RETURN_SUCCESS;
}

void bw_BrowserWindowImpl_new(
	bw_BrowserWindow* browser,
	bw_BrowserWindowSource _source,
	int width, int height,
	const bw_BrowserWindowOptions* browser_window_options,
	bw_BrowserWindowCreationCallbackFn callback,
	void* callback_data
) {
	// Create a copy of the source data because the data itself won't be available within the CreateWebViewControlAsync.Completed handler.
	std::string source( _source.data.data, _source.data.len );

	auto process = WebViewControlProcess();

	auto op = process.CreateWebViewControlAsync( reinterpret_cast<long>(browser->window->impl.handle), Rect( 0.0, 0.0, (float)width, (float)height ) );

	op.Completed([=](auto op, auto status) {

		if ( status == AsyncStatus::Error )
			BW_WIN32_ASSERT_HRESULT( op.ErrorCode() );

		if ( status == AsyncStatus::Completed ) {
			bw_BrowserWindowImpl impl;
			HandleData* handle_data = new HandleData( op.GetResults() );
			browser->impl.data = (void*)handle_data;

			// Allow calling back to us from js
			handle_data->control.Settings().IsScriptNotifyAllowed( true );
			// Is needed in order to be visible to the user
			handle_data->control.IsVisible( true );

			handle_data->control.ScriptNotify([=](auto, auto const& args) {
				std::string args_str = winrt::to_string( args.Value() );
				std::vector<std::string> parsed_args = parse_args( args_str.c_str() );
				std::vector<bw_CStrSlice> slices = args_slices( parsed_args );

				browser->external_handler( browser, slices[0], &slices[1], slices.size() - 1 );
			});

			// Inject javascript that creates the invoke_extern function
			handle_data->control.NavigationStarting([=](auto const& sender, auto const& args) {
				handle_data->control.AddInitializeScript(winrt::to_hstring(
					"(function() {"
						"window.invoke_extern = (...cmd) => {"
							"window.external.notify(cmd.join(String.fromCharCode(3)))"
						"}"
					"})();"
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
			callback( browser, callback_data );
		}
	});
}

void bw_BrowserWindowImpl_onResize( const bw_Window* window, unsigned int width, unsigned int height ) {

	auto bw = (bw_BrowserWindow*)window->user_data;
	auto hd = (HandleData*)bw->impl.data;

	Rect rect( 0.0, 0.0, (float)width, (float)height );
	hd->control.Bounds( rect );
}
