#ifndef BW_CEF_CLIENT_HANDLER_H
#define BW_CEF_CLIENT_HANDLER_H

#include <include/cef_client.h>
#include <include/cef_life_span_handler.h>
//#include <include/cef_print_handler.h>
#include <include/cef_v8.h>

#include "eval_callback_store.hpp"
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
			
			// Parameters
			unsigned int script_id = (unsigned int)message->GetArgumentList()->GetDouble( 0 );
			bool success = message->GetArgumentList()->GetBool( 1 );
			CefString result_str = message->GetArgumentList()->GetString( 2 );

			// Construct the callback result union
			bw::EvalCallbackResult cb_result;
			if (success) {
				cb_result.result = result_str;
			}
			else {
				bw_Err error = bw_Err_new_with_msg( 1, result_str.ToString().c_str() );

				cb_result.error = error;
			}

			// Invoke the callback!
			if ( !bw::eval_callback_store.invoke( script_id, success, cb_result ) ) {
				BW_ASSERT( false, "Eval callback doesn't exist!\n" );
			}

			// Free error
			if (!success) {
				bw_Err_free( &cb_result.error );
			}

			return true;
		}
		else
			fprintf(stderr, "Unknown process message received: %s\n", message->GetName().ToString().c_str() );

		return false;
	}

protected:
	IMPLEMENT_REFCOUNTING(ClientHandler);
};



#endif//BW_CEF_CLIENT_HANDLER_H
