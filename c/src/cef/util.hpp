#ifndef BW_CEF_H
#define BW_CEF_H

#include "../string.h"

#include <include/internal/cef_string.h>



CefString bw_cef_copyFromStrSlice( bw_CStrSlice slice );
size_t bw_cef_copyToCstr( const CefString& cef_string, char** cstr );
bw_CStrSlice bw_cef_copyToCStrSlice(const CefString& string);
bw_StrSlice bw_cef_copyToStrSlice(const CefString& string);



#endif//BW_CEF_H