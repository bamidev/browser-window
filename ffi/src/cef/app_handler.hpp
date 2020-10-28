#ifndef BW_CEF_APP_HANDLER_H
#define BW_CEF_APP_HANDLER_H

#include <include/cef_app.h>
#include <include/cef_client.h>
#include <include/cef_life_span_handler.h>
#include <include/cef_v8.h>

#include "external_invocation_handler.hpp"
#include "../assert.h"
#include "../application.h"



class AppHandler : public CefApp, public CefRenderProcessHandler {

	bw_Application* app;

public:
	AppHandler( bw_Application* app ) : app(app) {}

	virtual void OnContextCreated( CefRefPtr<CefBrowser> browser, CefRefPtr<CefFrame> frame, CefRefPtr<CefV8Context> context ) override {

		CefRefPtr<CefV8Value> object = context->GetGlobal();

		CefRefPtr<CefV8Handler> handler = new bw::ExternalInvocationHandler( browser );
		CefRefPtr<CefV8Value> func = CefV8Value::CreateFunction("invoke_extern", handler);

		bool result = object->SetValue( "invoke_extern", func, V8_PROPERTY_ATTRIBUTE_NONE );
		BW_ASSERT( result, "Unable to set invoke_extern function." );
	}

	virtual CefRefPtr<CefRenderProcessHandler> GetRenderProcessHandler() override {
		return this;
	}

	virtual bool OnProcessMessageReceived(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		CefProcessId _source_process,
		CefRefPtr<CefProcessMessage> message
	) override {

		// The message to execute some javascript, and return its output
		if ( message->GetName() == "eval-js" ) {
			int script_id = message->GetArgumentList()->GetInt( 0 );
			CefString js = message->GetArgumentList()->GetString( 1 );

			this->eval_js( browser, frame, js, script_id );

			return true;
		}
		else
			fprintf(stderr, "Unknown process message received: %s\n", message->GetName().ToString().c_str() );

		return false;
	}

	// Evaluate JavaScript, and send back a message to the main process with the result
	void eval_js(
		CefRefPtr<CefBrowser> browser,
		CefRefPtr<CefFrame> frame,
		const CefString& js,
		int script_id
	) {
		CefString script_url( "eval" );
		CefRefPtr<CefV8Value> ret_val;
		CefRefPtr<CefV8Exception> exception;

		bool result = frame->GetV8Context()->Eval( js, script_url, 0, ret_val, exception );

		// IPC message to be send to notify browser process of eval result
		CefRefPtr<CefProcessMessage> msg = CefProcessMessage::Create("eval-js-result");
		CefRefPtr<CefListValue> msg_args = msg->GetArgumentList();

		if ( !result ) {

			// The first parameter specifies the ID of the script that has executed
			msg_args->SetDouble( 0, script_id );
			// The second parameter specifies whether or not an error has resulted
			msg_args->SetBool( 1, false );
			// The third parameter specifies the error message
			msg_args->SetString( 2, exception->GetMessage() );
		}
		else {

			CefString result_string = ret_val->GetStringValue();

			// The first parameter specifies the ID of the script that has executed
			msg_args->SetDouble( 0, script_id );
			// The second parameter specifies whether or not an error has resulted
			msg_args->SetBool( 1, true );
			// The third parameter specifies the result formatted as a string
			msg_args->SetString( 2, result_string );
		}

		// Send the message back to the browser process
		frame->SendProcessMessage( PID_BROWSER, msg );
	}

protected:
	IMPLEMENT_REFCOUNTING(AppHandler);
};



#endif//BW_CEF_APP_HANDLER_H
