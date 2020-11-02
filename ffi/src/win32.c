#include "win32.h"

#include "debug.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <wchar.h>

#pragma comment(lib, "User32.lib")
#pragma comment(lib, "OleAut32.lib")



char* bw_win32_unhandledHresultMessage( bw_ErrCode code, void* data );
char* bw_win32_unknownHresultMessage( bw_ErrCode code, void* data );



void bw_win32_print_error() {

	DWORD code = GetLastError();

	wchar_t msg[512];
	FormatMessageW(
		FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
		NULL,
		code,
		0,
		msg,
		512,
		NULL
	);

	fwprintf(stderr, L"win32 assertion [%i]: %s\n", code, msg);

	// TODO: Print stack trace
}

void bw_win32_print_hresult_error( HRESULT hresult ) {

	wchar_t* message = 0;

	if ( FormatMessageW(
		FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
		NULL,
		hresult,
		MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
		&message,
		0, NULL )
	) {
		fwprintf( stderr, L"win32 hresult assertion [%x]: %s\n", hresult, message );

		free( message );
	}
	else {
		fprintf( stderr, "win32 hresult assertion with unknown message: %x\n", hresult );
	}
}

WCHAR* bw_win32_copyAsNewWstr( bw_CStrSlice slice ) {

	DWORD size = MultiByteToWideChar( CP_UTF8, 0, slice.data, (int)slice.len, 0, 0 );

	WCHAR* str = calloc( size + 1, sizeof(WCHAR) );
	if (str == 0) {
		return 0;
	}

	MultiByteToWideChar( CP_UTF8, 0, slice.data, (int)slice.len, str, size );
	str[size] = L'\0';

	return str;
}

char* bw_win32_copyAsNewCstr( bw_StrSlice str ) {
	char* new_str = malloc( str.len + 1 );

	memcpy( new_str, str.data, str.len );
	new_str[ str.len ] = '\0';

	return new_str;
}

char* bw_win32_copyWstrAsNewCstr( WCHAR* str ) {

	size_t len = wcslen( str );
	DWORD size_needed = WideCharToMultiByte( CP_UTF8, WC_COMPOSITECHECK | WC_DEFAULTCHAR | WC_NO_BEST_FIT_CHARS, str, (int)len, 0, 0, 0, 0 );

	char* cstr = calloc( size_needed + 1, sizeof( char ) );
	WideCharToMultiByte( CP_UTF8, WC_COMPOSITECHECK | WC_DEFAULTCHAR | WC_NO_BEST_FIT_CHARS, str, (int)len, cstr, size_needed, 0, 0 );
	cstr[ size_needed ] = '\0';

	return cstr;
}

bw_Err bw_win32_unhandledHresult( HRESULT hResult ) {

	TCHAR* message;

	if ( FACILITY_WINDOWS == HRESULT_FACILITY( hResult ) )
		hResult = HRESULT_CODE( hResult );

		if( FormatMessage(
			FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
			NULL,
			hResult,
			MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
			(LPWSTR)&message,
			0, NULL ) != 0
		) {

			bw_Err error;
			error.code = (bw_ErrCode)hResult;
			error.data = message;
			error.alloc_message = bw_win32_unhandledHresultMessage;
			return error;
		}
		else {
			bw_Err error;
			error.code = (bw_ErrCode)hResult;
			error.data = 0;	// No data is used here
			error.alloc_message = bw_win32_unknownHresultMessage;
			return error;
		}
}


char* bw_win32_unhandledHresultMessage( bw_ErrCode code, void* data ) {

	char* hresult_msg = bw_win32_copyWstrAsNewCstr( (WCHAR*)data );

	char* message = calloc( strlen("Unhandled win32 hresult error [0x00000000]: ") + strlen( hresult_msg ) + 9, sizeof( char ) );
	sprintf( message, "Unhandled win32 hresult error [0x%x]: %s", code, hresult_msg );

	free( hresult_msg );
	return message;
}

char* bw_win32_unknownHresultMessage( bw_ErrCode code, void* _ ) {

	char* message = calloc( strlen("Unknown win32 hresult error: 0x00000000") + 9, sizeof( char ) );

	sprintf( message, "Unknown win32 hresult error: 0x%x", code );

	return message;
}
