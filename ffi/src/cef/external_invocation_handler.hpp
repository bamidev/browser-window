#ifndef BW_CEF_EXTERNAL_INVOCATION_HANDLER
#define BW_CEF_EXTERNAL_INVOCATION_HANDLER

#include <include/cef_v8.h>
#include <optional>

#include "bw_handle_map.hpp"
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

					CefString string = this->v8ValueToString(*it);

					msg_args->SetString( index, string );
				}

				this->cef_browser->GetMainFrame()->SendProcessMessage( PID_BROWSER, msg );
			}

			return false;
		}

	protected:
		CefString boolToString( bool boolean ) {
			if ( boolean )
				return "true";
			// else
			return "false";
		}

		template <class V>
		CefString intoString( const V& value ) {
			std::string str = std::to_string( value );
			return CefString( str );
		}

		// Convert a javascript value into a string, for very basic compatibility purposes with the Rust application
		// Note: This function is not yet complete. Not all types are converted appropriately.
		CefString v8ValueToString( CefRefPtr<CefV8Value> val ) {

			// If undefined
			if ( val->IsUndefined() )
				return "undefined";

			// If null
			if ( val->IsNull() )
				return "null";

			// If string
			if ( val->IsString() )
				return val->GetStringValue();

			// If boolean
			if ( val->IsBool() )
 				return this->boolToString( val->GetBoolValue() );

			// If integer
			if ( val->IsInt() )
				return this->intoString( val->GetIntValue() );

			// If unsigned integer
			if ( val->IsUInt() )
				return this->intoString( val->GetUIntValue() );

			// If unsigned integer
			if ( val->IsDouble() )
				return this->intoString( val->GetDoubleValue() );

			// If object (unsupported)
			if ( val->IsObject() )
				return "[object]";

			// If array (unsupported)
			if ( val->IsArray() )
				return "[array]";

				// If array (unsupported)
			if ( val->IsDate() )
				return "[date]";

			// If exception (unsupported)
			if ( val->IsFunction() )
				return "[function]";

			// If type is not accounted for, return this string:
			return "[unknown type]";
		}

		IMPLEMENT_REFCOUNTING(ExternalInvocationHandler);
	};
}



#endif//BW_CEF_EXTERNAL_INVOCATION_HANDLER
