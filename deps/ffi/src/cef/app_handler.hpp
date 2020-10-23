#ifndef BW_CEF_APP_HANDLER_H
#define BW_CEF_APP_HANDLER_H

#include <include/cef_app.h>
#include <include/cef_client.h>
#include <include/cef_life_span_handler.h>
//#include <include/cef_print_handler.h>
#include <include/cef_v8.h>

#include "../assert.h"
#include "../application.h"



class AppHandler : public CefApp, public CefRenderProcessHandler {

	bw_Application* app;

public:
	AppHandler( bw_Application* app ) : app(app) {}

	virtual void OnContextCreated( CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, CefRefPtr<CefV8Context> context ) override {

		CefString code = "window.external.invoke = function( cmd ) {};";
		CefString script_url("eval");
		CefRefPtr<CefV8Value> ret_val;
		CefRefPtr<CefV8Exception> exc;

		bool result = context->Eval( code, script_url, 0, ret_val, exc );
		BW_ASSERT( result, "Unable to execute Javascript to initialize the window.external.invoke function: %s", exc->GetMessage() );
	}

	virtual CefRefPtr<CefRenderProcessHandler> GetRenderProcessHandler() override {
		return this;
	}

	virtual bool OnProcessMessageReceived(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		CefProcessId source_process,
		CefRefPtr<CefProcessMessage> message
	) override {

		if ( message->GetName() == "eval-js" ) {
			int script_id = message->GetArgumentList()->GetInt( 0 );
			CefString js = message->GetArgumentList()->GetString( 1 );

			this->evalJs( browser, frame, js, script_id );

			return true;
		}
		else if ( message->GetName() == "eval-js-result" ) {
			int script_id = message->GetArgumentList()->GetInt( 0 );
			bool success = message->GetArgumentList()->GetBool( 1 );
			CefString result = message->GetArgumentList()->GetString( 2 );

			fprintf(stderr, "Test: %s\n", result.ToString().c_str());

			return true;
		}

		return false;
	}

	// Evaluate JavaScript, and send back a message to the main process with the result
	void evalJs(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		const CefString& js,
		int script_id
	) {

		CefString script_url( "eval" );
		CefRefPtr<CefV8Value> ret_val;
		CefRefPtr<CefV8Exception> exception;

		bool result = frame->GetV8Context()->Eval( js, script_url, 0, ret_val, exception );

		CefRefPtr<CefProcessMessage> msg = CefProcessMessage::Create("eval-js-result");
		CefRefPtr<CefListValue> msg_args = msg->GetArgumentList();
		if ( !result ) {

			CefString error_msg = "error";//bw_cef_v8exc_to_string( exception );

			// The first parameter specifies the ID of the script that has executed
			msg_args->SetInt( 0, script_id );
			// The second parameter specifies whether or not an error has resulted
			msg_args->SetBool( 1, false );
			// The third parameter specifies the error message
			msg_args->SetString( 2, error_msg );
		}
		else {

			CefString result_string = ret_val->GetStringValue();

			// The first parameter specifies the ID of the script that has executed
			msg_args->SetInt( 0, script_id );
			// The second parameter specifies whether or not an error has resulted
			msg_args->SetBool( 1, true );
			// The third parameter specifies the result string
			msg_args->SetString( 2, result_string );
		}

		// Send the message back to the browser process
		frame->SendProcessMessage( PID_BROWSER, msg );
	}

protected:
	IMPLEMENT_REFCOUNTING(AppHandler);
};



#endif//BW_CEF_APP_HANDLER_H
