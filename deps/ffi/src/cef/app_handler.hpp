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
		CefString code = "window.external.invoke = function( cmd, args... ) {}";
		CefString script_url("eval");
		CefRefPtr<CefV8Value> ret_val;
		CefRefPtr<CefV8Exception> exc;

		bool result = context->Eval( code, script_url, 0, ret_val, exc );
		BW_ASSERT( result, "Unable to execute Javascript to initialize the window.external.invoke function: %s", exc->GetMessage() );
	}

protected:
	IMPLEMENT_REFCOUNTING(AppHandler);
};



#endif//BW_CEF_APP_HANDLER_H
