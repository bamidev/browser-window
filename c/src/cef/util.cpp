#include "util.hpp"

#include <string>



CefString bw_cef_copyToString( bw_CStrSlice slice ) {

	std::string temp( slice.data, slice.len );
	CefString string( temp );

	return string;
}

size_t bw_cef_copyToCstr( const CefString& cef_string, char** cstr ) {
	std::string temp = cef_string.ToString();

	*cstr = (char*)malloc(temp.length());
	memcpy(*cstr, temp.c_str(), temp.length());

	return temp.length();
}