#ifndef BW_CEF_CLIENT_HANDLER_H
#define BW_CEF_CLIENT_HANDLER_H

#include <include/cef_client.h>
#include <include/cef_life_span_handler.h>
//#include <include/cef_print_handler.h>
#include <include/cef_v8.h>

#include "../application.h"



class ClientHandler : public CefClient, public CefLifeSpanHandler, public CefV8Handler {

	bw_Application* app;
	unsigned int browser_count;

public:
	ClientHandler( bw_Application* app ) : app(app), browser_count(0) {}

	virtual CefRefPtr<CefLifeSpanHandler> GetLifeSpanHandler() override {
		return this;
	}

	//virtual CefRefPtr<CefPrintHandler> GetPrintHandler() override {
	//	return this;
	//}

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

	// Virutal on CefV8Handler
	bool Execute( const CefString& name, CefRefPtr<CefV8Value> object, const CefV8ValueList& arguments, CefRefPtr< CefV8Value >& retval, CefString& exception ) override {
		//TODO: Call handler..
		return false;
	}

protected:
	IMPLEMENT_REFCOUNTING(ClientHandler);
};



#endif//BW_CEF_CLIENT_HANDLER_H
