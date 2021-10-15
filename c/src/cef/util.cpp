#include "util.hpp"

#include <string>



CefString bw_cef_copyFromStrSlice( bw_CStrSlice slice ) {

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

bw_CStrSlice bw_cef_copyToCStrSlice(const CefString& string) {
	bw_CStrSlice slice;

	std::string temp = string.ToString();

	char* data = (char*)malloc(temp.length());
	slice.data = data;
	slice.len = temp.length();
	return slice;
}

bw_StrSlice bw_cef_copyToStrSlice(const CefString& string) {
	bw_StrSlice slice;

	std::string temp = string.ToString();

	size_t len = temp.length();
	char* data = (char*)malloc(len);
	memcpy(data, temp.c_str(), len);
	slice.data = data;
	slice.len = len;
	return slice;
}