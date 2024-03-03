#ifndef BROWSER_WINDOW_WIN32_H
#define BROWSER_WINDOW_WIN32_H

#ifdef __cplusplus
extern "C" {
#endif

#include <assert.h>

#include "err.h"
#include "string.h"


// Some definitions as defined in windef.h:
// https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types
// Because including windef.h with MinGW can cause some issues
typedef unsigned char BYTE;
typedef unsigned long DWORD;
typedef long HRESULT;
typedef wchar_t WCHAR;


#define BW_WIN32_PANIC_LAST_ERROR \
	{ bw_win32_print_error( GetLastError() ); assert(0); }
#define BW_WIN32_PANIC_HRESULT( HRESULT ) \
	{ bw_win32_print_hresult_error( HRESULT ); assert(0); }
#define BW_WIN32_ASSERT_SUCCESS \
    { DWORD err = GetLastError(); if ( err != 0 ) { bw_win32_print_error( err ); assert(0); } }



char* bw_win32_copyWstrAsNewCstr( const WCHAR* str );
char* bw_win32_copyAsNewCstr( bw_CStrSlice str );

size_t bw_win32_copyAsNewUtf8Str( const WCHAR* string, char** output );

/// Copies the given string into a newly allocated BSTR (widestring).
/// Make sure to free it with SysFreeString.
WCHAR* bw_win32_copyAsNewWstr( bw_CStrSlice str );

void bw_win32_print_error( DWORD code );

void bw_win32_print_hresult_error( HRESULT hresult );

void bw_win32_copyWcharIntoSlice( const WCHAR* input, bw_StrSlice output );

bw_Err bw_win32_unhandledHresult( HRESULT hresult );



#ifdef __cplusplus
}	// extern "C"
#endif

#endif//BROWSER_WINDOW_WIN32_H
