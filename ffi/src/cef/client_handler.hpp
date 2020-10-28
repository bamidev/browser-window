#ifndef BW_CEF_CLIENT_HANDLER_H
#define BW_CEF_CLIENT_HANDLER_H

#include <include/cef_client.h>
#include <include/cef_life_span_handler.h>
#include <include/cef_v8.h>

#include "bw_handle_map.hpp"
#include "../application.h"



class ClientHandler : public CefClient, public CefLifeSpanHandler {

	bw_Application* app;
	unsigned int browser_count;

public:
	ClientHandler( bw_Application* app ) : app(app), browser_count(0) {}

	virtual CefRefPtr<CefLifeSpanHandler> GetLifeSpanHandler() override {
		return this;
	}

	// Virutal on CefLifeSpanHandler
	virtual bool DoClose(CefRefPtr<CefBrowser> browser) override { return false; }
	virtual bool OnBeforePopup( CefRefPtr< CefBrowser > browser, CefRefPtr< CefFrame > frame, const CefString& target_url, const CefString& target_frame_name, CefLifeSpanHandler::WindowOpenDisposition target_disposition, bool user_gesture, const CefPopupFeatures& popupFeatures, CefWindowInfo& windowInfo, CefRefPtr< CefClient >& client, CefBrowserSettings& settings, CefRefPtr< CefDictionaryValue >& extra_info, bool* no_javascript_access ) override { return true; }

	virtual void OnAfterCreated( CefRefPtr<CefBrowser> browser ) override {
		this->browser_count += 1;
	}

	virtual void OnBeforeClose(CefRefPtr<CefBrowser> browser) override {
		this->browser_count -= 1;

		// Fetch our handle
		std::optional<bw_BrowserWindow*> result = bw::bw_handle_map.fetch( browser );
		BW_ASSERT( result.has_value(), "Link between CEF's browser handle and our handle does not exist!\n" );
		bw_BrowserWindow* handle = *result;

		handle->window->closed = true;

		// If the handle has been dropped, free the browser window
		if ( !handle->inner.handle_is_used ) {
			bw_BrowserWindow_free( handle );
		}

		// If the last browser window is now closed, we exit the application
		if ( this->browser_count == 0 ) {
			bw_Application_exit( this->app, 0 );
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
		else
			fprintf(stderr, "Unknown process message received: %s\n", message->GetName().ToString().c_str() );

		return false;
	}

protected:

	void onEvalJsResultReceived(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		CefProcessId source_process,
		CefRefPtr<CefProcessMessage> message
	) {
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
		CefRefPtr<CefFrame> _frame,
		CefProcessId _source_process,
		CefRefPtr<CefProcessMessage> msg
	) {
		// Obtain our browser window handle
		std::optional<bw_BrowserWindow*> _bw_handle = bw::bw_handle_map.fetch( browser );
		BW_ASSERT( _bw_handle.has_value(), "Link between CEF's browser handle and our handle does not exist!\n" );
		bw_BrowserWindow* our_handle = *_bw_handle;

		auto params = msg->GetArgumentList();

		// This argument is the command string
		CefString cmd = params->GetString( 0 );

		// TODO: Obtain all extra parameters

		// Convert cmd from CefString to bw_CStrSlice
		std::string cmd_str = cmd.ToString();
		bw_CStrSlice cmd_str_slice = {
			cmd_str.length(),
			cmd_str.c_str()
		};
		// TODO: Move this conversion into its own function

		our_handle->external_handler( our_handle, cmd_str_slice, 0, 0 );
	}

	IMPLEMENT_REFCOUNTING(ClientHandler);
};



#endif//BW_CEF_CLIENT_HANDLER_H
