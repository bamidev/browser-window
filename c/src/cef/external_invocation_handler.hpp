#ifndef BW_CEF_EXTERNAL_INVOCATION_HANDLER
#define BW_CEF_EXTERNAL_INVOCATION_HANDLER

#include <include/cef_v8.h>
#include <optional>

#include "bw_handle_map.hpp"
#include "v8_to_string.hpp"
#include "../assert.h"



namespace bw {

	class ExternalInvocationHandler : public CefV8Handler {
		CefRefPtr<CefBrowser> cef_browser;

	public:
		ExternalInvocationHandler( CefRefPtr<CefBrowser> browser ) : cef_browser(browser) {}

		virtual bool Execute(
			const CefString& name,
			CefRefPtr<CefV8Value> object,
			const CefV8ValueList& arguments,
			CefRefPtr<CefV8Value>& retval,
			CefString& exception
		) override  {
			(void)(object);
			(void)(retval);
			(void)(exception);

			if ( name == "invoke_extern" ) {

				CefRefPtr<CefProcessMessage> msg = CefProcessMessage::Create("invoke-handler");
				CefRefPtr<CefListValue> msg_args = msg->GetArgumentList();

				// Convert all function arguments to strings
				size_t index = 0;
				for ( auto it = arguments.begin(); it != arguments.end(); it++, index++ ) {

					if (index == 0) {
						msg_args->SetString(0, (*it)->GetStringValue());
					} else {
						CefString string = V8ToString::convert(*it);
						msg_args->SetString( index, string );
					}
				}

				this->cef_browser->GetMainFrame()->SendProcessMessage( PID_BROWSER, msg );
			}

			return false;
		}

	protected:
		IMPLEMENT_REFCOUNTING(ExternalInvocationHandler);
	};
}



#endif//BW_CEF_EXTERNAL_INVOCATION_HANDLER
