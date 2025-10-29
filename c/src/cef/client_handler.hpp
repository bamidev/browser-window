#ifndef BW_CEF_CLIENT_HANDLER_H
#define BW_CEF_CLIENT_HANDLER_H

#include <include/cef_client.h>
#include <include/cef_download_handler.h>
#include <include/cef_life_span_handler.h>
#include <include/cef_load_handler.h>
#include <include/cef_request_handler.h>
#include <include/cef_v8.h>
#include <string>
#include <vector>

#include "bw_handle_map.hpp"
#include "util.hpp"
#include "../application.h"
#include "../common.h"



struct ExternalInvocationHandlerData {
	bw_BrowserWindow* bw;
	std::string cmd;
	std::vector<std::string> params;
};

class ClientHandler :
	public CefClient,
	public CefDisplayHandler,
	public CefDownloadHandler,
	public CefRequestHandler,
	public CefLifeSpanHandler,
	public CefLoadHandler
{

	bw_Application* app;

public:
	ClientHandler( bw_Application* app ) : app(app) {}

	CefRefPtr<CefDisplayHandler> GetDisplayHandler() override { return this; }
	CefRefPtr<CefDownloadHandler> GetDownloadHandler() override { return this; }
	CefRefPtr<CefLifeSpanHandler> GetLifeSpanHandler() override { return this; }
	CefRefPtr<CefLoadHandler> GetLoadHandler() override { return this; }
	CefRefPtr<CefRequestHandler> GetRequestHandler() override { return this; }

	void OnAddressChange(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, const CefString& url) override {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			bw_CStrSlice slice = bw_cef_copyToCStrSlice(url);
			auto bw_info = bw_info_opt.value();
			bw_Event_fire(&bw_info->handle->events.on_address_changed, (void*)&slice);
			bw_string_freeC(slice);
		}
	}

#ifdef BW_WIN32
	void
#else
	bool
#endif
	OnBeforeDownload(CefRefPtr<CefBrowser> browser, CefRefPtr<CefDownloadItem> download_item, const CefString& suggested_name, CefRefPtr<CefBeforeDownloadCallback> callback ) {
#ifndef BW_WIN32
		return false;
#endif
	}

	bool OnConsoleMessage( CefRefPtr< CefBrowser > browser, cef_log_severity_t level, const CefString& message, const CefString& source, int line ) override {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			bw_CStrSlice slice = bw_cef_copyToCStrSlice(message);
			auto bw_info = bw_info_opt.value();
			bw_Event_fire(&bw_info->handle->events.on_console_message, (void*)&slice);
			bw_string_freeC(slice);
		}
		return false;
	}

	void OnFaviconURLChange( CefRefPtr< CefBrowser > browser, const std::vector< CefString >& icon_urls ) override {

	}

	void OnFullscreenModeChange(CefRefPtr<CefBrowser> browser, bool fullscreen) override {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			auto bw_info = bw_info_opt.value();
			bw_Event_fire(&bw_info->handle->events.on_fullscreen_mode_changed, (void*)&fullscreen);
		}
	}

	virtual void OnLoadEnd(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, int httpStatusCode) override {
		BW_ERR_DECLARE_SUCCESS(error);
		this->invokeCreationCallback(browser, error);
	}

	bw_Err convertLoadResult(CefLoadHandler::ErrorCode errorCode, const CefString& errorText) {
		if (errorCode == 0) {
			BW_ERR_DECLARE_SUCCESS(error);
			return error;
		} else {
			bw_Err error = bw_Err_new_with_msg(errorCode, errorText.ToString().c_str());
			return error;
		}
	}

	void invokeCreationCallback(CefRefPtr<CefBrowser> browser, bw_Err error) {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);

		if (bw_info_opt.has_value()) {
			auto bw_info = bw_info_opt.value();
			auto callback_opt = &bw_info->callback;
			if (callback_opt->has_value()) {
				auto value = callback_opt->value();
				callback_opt->reset();
				value.callback(bw_info->handle, value.data);
			}

			bw_Event_fire(&bw_info->handle->events.on_navigation_end, (void*)&error);
			bw_Err_free(&error);
		} else {
#ifndef NDEBUG
			printf("Unable to obtain browser info for browser.\n");
#endif
		}
	}

	virtual void OnLoadError(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, CefLoadHandler::ErrorCode errorCode, const CefString& errorText, const CefString& failedUrl) override {
		bw_Err error = this->convertLoadResult(errorCode, errorText);
		this->invokeCreationCallback(browser, error);
	}

	virtual void OnLoadStart(CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, CefLoadHandler::TransitionType transition_type ) override {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			auto bw_info = bw_info_opt.value();
			bw_CStrSlice slice = { 0, 0 };
			bw_Event_fire(&bw_info->handle->events.on_navigation_start, (void*)&slice);
		}
	}

	void OnLoadingProgressChange(CefRefPtr<CefBrowser> browser, double progress) override {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			auto bw_info = bw_info_opt.value();
			bw_Event_fire(&bw_info->handle->events.on_loading_progress_changed, (void*)&progress);
		}
	}

	void OnStatusMessage(CefRefPtr<CefBrowser> browser, const CefString& value) {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			bw_CStrSlice slice = bw_cef_copyToCStrSlice(value);
			auto bw_info = bw_info_opt.value();
			bw_Event_fire(&bw_info->handle->events.on_status_message, (void*)&slice);
			bw_string_freeC(slice);
		}
	}

	virtual void OnTitleChange(CefRefPtr<CefBrowser> browser, const CefString& title) override {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			auto bw_info = bw_info_opt.value();
			bw_CStrSlice slice = bw_cef_copyToCStrSlice(title);
			bw_Event_fire(&bw_info->handle->events.on_page_title_changed, (void*)&slice);
			bw_string_freeC(slice);
		}
	}

	virtual bool OnTooltip(CefRefPtr<CefBrowser> browser, CefString& tooltip) override {
		std::optional<bw::BrowserInfo*> bw_info_opt = bw::bw_handle_map.fetch(browser);
		if (bw_info_opt.has_value()) {
			auto bw_info = bw_info_opt.value();
			bw_CStrSlice slice = bw_cef_copyToCStrSlice(tooltip);
			BOOL result = bw_Event_fire(&bw_info->handle->events.on_tooltip, (void*)&slice);
			bw_string_freeC(slice);
			return result;
		}
		return false;
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

	static void externalInvocationHandlerFunc( bw_Application* app, void* data );

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
		std::optional<bw::BrowserInfo*> bw_info = bw::bw_handle_map.fetch(browser);
		BW_ASSERT( bw_info.has_value(), "Link between CEF's browser handle and our handle does not exist!\n" );
		bw_BrowserWindow* our_handle = bw_info.value()->handle;

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

	IMPLEMENT_REFCOUNTING(ClientHandler);
};



#endif//BW_CEF_CLIENT_HANDLER_H
