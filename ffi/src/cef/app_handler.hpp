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
			auto msg_args = message->GetArgumentList();

			// Javascript to execute
			CefString js = msg_args->GetString( 0 );

			// Browser window handle
			CefRefPtr<CefBinaryValue> bw_bin = msg_args->GetBinary( 1 );
			CefRefPtr<CefBinaryValue> cb_bin = msg_args->GetBinary( 2 );
			CefRefPtr<CefBinaryValue> user_data_bin = msg_args->GetBinary( 3 );


			this->eval_js( browser, frame, js, bw_bin, cb_bin, user_data_bin );

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
		CefRefPtr<CefBinaryValue> bw_handle_binary,
		CefRefPtr<CefBinaryValue> callback_binary,
		CefRefPtr<CefBinaryValue> user_data_binary
	) {
		CefString script_url( "eval" );
		CefRefPtr<CefV8Value> ret_val;
		CefRefPtr<CefV8Exception> exception;

		bool result = frame->GetV8Context()->Eval( js, script_url, 0, ret_val, exception );

		// IPC message to be send to notify browser process of eval result
		CefRefPtr<CefProcessMessage> msg = CefProcessMessage::Create("eval-js-result");
		CefRefPtr<CefListValue> msg_args = msg->GetArgumentList();

		if ( !result ) {

			// The first parameter specifies whether or not an error has resulted
			msg_args->SetBool( 0, false );
			// The second parameter specifies the error message
			msg_args->SetString( 1, exception->GetMessage() );
		}
		else {

			CefString result_string = ret_val->GetStringValue();

			// The first parameter specifies whether or not an error has resulted
			msg_args->SetBool( 0, true );
			// The second parameter specifies the result formatted as a string
			msg_args->SetString( 1, result_string );
		}

		// Send along the binaries of the callback data
		msg_args->SetBinary( 2, bw_handle_binary );
		msg_args->SetBinary( 3, callback_binary );
		msg_args->SetBinary( 4, user_data_binary );

		// Send the message back to the browser process
		frame->SendProcessMessage( PID_BROWSER, msg );
	}

protected:
	IMPLEMENT_REFCOUNTING(AppHandler);
};



#endif//BW_CEF_APP_HANDLER_H
