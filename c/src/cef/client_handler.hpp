#ifndef BW_CEF_CLIENT_HANDLER_H
#define BW_CEF_CLIENT_HANDLER_H

#include <include/cef_client.h>
#include <include/cef_life_span_handler.h>
#include <include/cef_v8.h>
#include <string>
#include <vector>
#include <include/cef_load_handler.h>

#include "bw_handle_map.hpp"
#include "../application.h"
#include "../common.h"



struct ExternalInvocationHandlerData {
	bw_BrowserWindow* bw;
	std::string cmd;
	std::vector<std::string> params;
};

class ClientHandler : public CefClient, public CefLifeSpanHandler, public CefLoadHandler {

	bw_Application* app;

public:
	ClientHandler( bw_Application* app ) : app(app) {}

	virtual CefRefPtr<CefLifeSpanHandler> GetLifeSpanHandler() override { return this; }

	virtual CefRefPtr<CefLoadHandler> GetLoadHandler() override { return this; }

	virtual void OnLoadEnd(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, int httpStatusCode) override {
		std::optional<bw::BrowserInfo> bw_info_opt = bw::bw_handle_map.fetch(browser);
		BW_ASSERT(bw_info_opt.has_value(), "Link between CEF's browser handle and our handle does not exist!\n");
		auto bw_info = bw_info_opt.value();
		auto callback_opt = bw_info.callback;
		if (callback_opt.has_value()) {
			auto value = callback_opt.value();
			value.callback(bw_info.handle, value.data);
		}
	}

	virtual void OnLoadError(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, CefLoadHandler::ErrorCode errorCode, const CefString& errorText, const CefString& failedUrl) override {
		std::optional<bw::BrowserInfo> bw_info_opt = bw::bw_handle_map.fetch(browser);
		BW_ASSERT(bw_info_opt.has_value(), "Link between CEF's browser handle and our handle does not exist!\n");
		auto bw_info = bw_info_opt.value();
		auto callback_opt = bw_info.callback;
		if (callback_opt.has_value()) {
			auto value = callback_opt.value();
			value.callback(bw_info.handle, value.data);
		}
	}

	virtual bool OnProcessMessageReceived(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		CefProcessId source_process,
		CefRefPtr<CefProcessMessage> message
	) override {

		// The message to reveal the result of some javascript code
		if ( message->GetName() == "eval-js-result" ) {
			this->onEvalJsResultReceived( browser, frame, source_process, message );
			return true;
		}
		// The message to send data from within javascript to application code
		else if ( message->GetName() == "invoke-handler" ) {
			this->onInvokeHandlerReceived( browser, frame, source_process, message );
			return true;
		}
		// The OnBrowserCreated event is fired on another process, so we need to catch it here and
		// update the bw_handle_map in this process.
		else if ( message->GetName() == "on-browser-created" ) {
			this->onBrowserCreated( browser, frame, source_process, message );
			return true;
		}
		else
			fprintf(stderr, "Unknown process message received: %s\n", message->GetName().ToString().c_str() );

		return false;
	}

protected:

	static void externalInvocationHandlerFunc( bw_Application* app, void* data );

	void onBrowserCreated(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame>,
		CefProcessId,
		CefRefPtr<CefProcessMessage> msg
	) {
		auto args = msg->GetArgumentList();

		// Process message arguments
		bw_BrowserWindow* bw_handle;
		args->GetBinary( 0 )->GetData( (void*)&bw_handle, sizeof( bw_handle ), 0 );
		bw_BrowserWindowCreationCallbackFn callback;
		args->GetBinary( 1 )->GetData( (void*)&callback, sizeof( callback ), 0 );
		void* callback_data;
		args->GetBinary( 2 )->GetData( (void*)&callback_data, sizeof( callback_data ), 0 );
		bool dev_tools_enabled = args->GetBool( 3 );

		// Make a copy on the heap to store in our handle
		CefRefPtr<CefBrowser>* cef_ptr = new CefRefPtr<CefBrowser>( browser );
		bw_handle->impl.cef_ptr = (void*)cef_ptr;

		// Store a link with the cef browser handle and our handle in a global map
		bw::bw_handle_map.store(*cef_ptr, bw_handle, callback, callback_data);

		// Open dev-tools window
		if ( dev_tools_enabled )
			this->openDevTools( bw_handle, browser->GetHost() );
	}

	void onEvalJsResultReceived(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		CefProcessId source_process,
		CefRefPtr<CefProcessMessage> message
	) {
		// Unused parameters
		(void)(browser);
		(void)(frame);
		(void)(source_process);

		auto msg_args = message->GetArgumentList();

		// Parameters
		bool success = msg_args->GetBool( 0 );
		CefString cef_result = msg_args->GetString( 1 );
		std::string result = cef_result.ToString();

		// Browser window handle
		bw_BrowserWindow* bw_handle;
		CefRefPtr<CefBinaryValue> bw_handle_bin = msg_args->GetBinary( 2 );
		bw_handle_bin->GetData( (void*)&bw_handle, sizeof( bw_handle ), 0 );

		// Callback function
		bw_BrowserWindowJsCallbackFn callback;
		CefRefPtr<CefBinaryValue> cb_bin = msg_args->GetBinary( 3 );
		cb_bin->GetData( (void*)&callback, sizeof( callback ), 0 );

		// User data for the callback function
		void* user_data;
		CefRefPtr<CefBinaryValue> user_data_bin = msg_args->GetBinary( 4 );
		user_data_bin->GetData( (void*)&user_data, sizeof( user_data ), 0 );

		// FIXME: call the relevant code on the right thread...

		// // Invoke the callback with either a result string or an error
		if (success) {
			callback( bw_handle, user_data, result.c_str(), 0 );
		}
		else {
			bw_Err error = bw_Err_new_with_msg( 1, result.c_str() );

			callback( bw_handle, user_data, 0, &error );
			bw_Err_free( &error );
		}
	}

	void onInvokeHandlerReceived(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		CefProcessId source_process,
		CefRefPtr<CefProcessMessage> msg
	) {
		(void)(frame);
		(void)(source_process);

		// Obtain our browser window handle
		std::optional<bw::BrowserInfo> bw_info = bw::bw_handle_map.fetch(browser);
		BW_ASSERT( bw_info.has_value(), "Link between CEF's browser handle and our handle does not exist!\n" );
		bw_BrowserWindow* our_handle = bw_info.value().handle;

		auto msg_args = msg->GetArgumentList();

		// This argument is the command string
		CefString cmd = msg_args->GetString( 0 );
		std::string cmd_str = cmd.ToString();

		// All next message arguments are the arguments of the command
		std::vector<std::string> params; params.reserve( msg_args->GetSize() - 1 );
		for ( size_t i = 1; i < msg_args->GetSize(); i++ ) {
			std::string param = msg_args->GetString( i ).ToString();

			params.push_back( param );
		}

		// Dispatch the invocation of the external handler to the thread from which the BrowserWindow main loop runs.
		auto dispatch_data = new ExternalInvocationHandlerData {
			our_handle,
			cmd_str,
			params
		};
		bw_Application_dispatch(
			our_handle->window->app,
			externalInvocationHandlerFunc,
			dispatch_data
		);
	}

	void openDevTools( bw_BrowserWindow* bw, const CefRefPtr<CefBrowserHost>& host ) {
#ifdef BW_WIN32

		CefRefPtr<CefClient> client;
		CefBrowserSettings settings;
		CefPoint point;

		CefWindowInfo info;
		info.SetAsPopup( bw->window->impl.handle, "Dev Tools" );

		host->ShowDevTools( info, client, settings, point );
#endif

		// FIXME: Implement dev tools for non windows systems
	}

	IMPLEMENT_REFCOUNTING(ClientHandler);
};



#endif//BW_CEF_CLIENT_HANDLER_H
