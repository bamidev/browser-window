#ifndef BW_CEF_V8_TO_STRING_HPP
#define BW_CEF_V8_TO_STRING_HPP

#include <string>
#include <vector>


class V8ToString {
public:

	// Convert a javascript value into a string, for very basic compatibility purposes with the Rust application
	// Note: This function is not yet complete. Not all types are converted appropriately.
	static CefString convert( CefRefPtr<CefV8Value> val ) {

		// If undefined
		if ( val->IsUndefined() )
			return "undefined";

		// If null
		if ( val->IsNull() )
			return "null";

		// If string
		if ( val->IsString() ) {
			std::string string = "\"";
			// TODO: Escape the string value:
			string += val->GetStringValue();
			string += "\"";
			return string;
		}

		// If boolean
		if ( val->IsBool() )
			return boolToString( val->GetBoolValue() );

		// If integer
		if ( val->IsInt() )
			return intoString( val->GetIntValue() );

		// If unsigned integer
		if ( val->IsUInt() )
			return intoString( val->GetUIntValue() );

		// If unsigned integer
		if ( val->IsDouble() )
			return intoString( val->GetDoubleValue() );

		// If array
		if ( val->IsArray() ) {
			std::string string = "[";
			for (int i = 0; i < val->GetArrayLength(); i++) {
				if (i != 0) {
					string += ",";
				}
				string += convert(val->GetValue(i));
			}
			string += "]";
			return string;
		}

		// If object
		if ( val->IsObject() ) {
			std::vector<CefString> keys;
			val->GetKeys(keys);
			std::string string = "{";
			for (size_t i = 0; i < keys.size(); i++) {
				std::string key = keys[i];
				if (i != 0) {
					string += ",";
				}
				string += key + ":" + convert(val->GetValue(i)).ToString();
			}
			string += "}";
			return string;
		}

		if ( val->IsDate() )
			return "date";

		// If exception (unsupported)
		if ( val->IsFunction() )
			return "function";

		// If type is not accounted for, return this string:
		return "unknown type";
	}

protected:

	static CefString boolToString( bool boolean ) {
		if ( boolean )
			return "true";
		// else
		return "false";
	}

	template <class V>
	static CefString intoString( const V& value ) {
		std::string str = std::to_string( value );
		return CefString( str );
	}
};



#endif//BW_CEF_V8_TO_STRING_HPP