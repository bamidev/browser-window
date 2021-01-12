#include "util.hpp"

#include <string>



CefString bw_cef_copyToString( bw_CStrSlice slice ) {

	std::string temp( slice.data, slice.len );
	CefString string( temp );

	return string;
}