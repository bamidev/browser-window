#ifndef BW_CEF_EXCEPTION_HPP
#define BW_CEF_EXCEPTION_HPP

#include <include/cef_v8.h>

#include "../err.h"



// Creates a new bw_Err instance that translates the CefV8Exception
bw_Err bw_cef_v8exc_to_bwerr( const CefRefPtr<CefV8Exception>& exc );



#endif//BW_CEF_EXCEPTION_HPP
