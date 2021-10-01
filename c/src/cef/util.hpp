#ifndef BW_CEF_H
#define BW_CEF_H

#include "../string.h"

#include <include/internal/cef_string.h>



CefString bw_cef_copyToString( bw_CStrSlice slice );
size_t bw_cef_copyToCstr( const CefString& cef_string, char** cstr );



#endif//BW_CEF_H