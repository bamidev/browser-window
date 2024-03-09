#ifndef BW_STRING_H
#define BW_STRING_H

#ifdef __cplusplus
extern "C" {
#endif

#ifndef BW_BINDGEN
#include <stddef.h>
#else
#ifdef BW_WIN32
typedef unsigned long long size_t;
#else
#include <stddef.h>
#endif
#endif


/// A 'string slice'
/// Points to a mutable, non-zero-terminated, UTF-8 encoded string.
/// Using rust's string's memory layout.
typedef struct bw_StrSlice {
	size_t len;
	char* data;
} bw_StrSlice;

/// A 'const string slice'
/// Points to a immutable, non-zero-terminated, UTF-8 encoded string.
/// Using rust's string's memory layout.
typedef struct bw_CStrSlice {
	size_t len;
	const char* data;
} bw_CStrSlice;



/// Copies the string from the given `bw_CStrSlice` to a C compatible, nul-terminated string.
char* bw_string_copyAsNewCstr( bw_CStrSlice str );

/// Frees the string allocated with any of the functions of this module.
void bw_string_freeCstr( char* str );
void bw_string_free(bw_StrSlice str);
void bw_string_freeC(bw_CStrSlice str);



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_STRING_H
