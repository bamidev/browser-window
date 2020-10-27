#include "exception.hpp"



bw_Err bw_cef_v8exc_to_bwerr( const CefRefPtr<CefV8Exception>& exc ) {

	bw_Err err;
	err.code = 1;	// CefV8Exception doesn't provide an error code
	err.alloc_message = bw_Err_msg_string;

	// Allocate a string with the exception message
	// TODO: Conversion can be done more efficiently by doing the conversion from wide string to normal string in the newly allocated buffer immediately
	std::string error_msg = exc->GetMessage().ToString();
	void* copy = malloc( error_msg.length() + 1 );
	memcpy( copy, error_msg.c_str(), error_msg.length() + 1 );

	err.data = copy;
	return err;
}
