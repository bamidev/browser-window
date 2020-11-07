#ifndef BW_STRING_H
#define BW_STRING_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>



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



char* bw_string_copyAsNewCstr( bw_CStrSlice str );
void bw_string_freeCstr( char* str );



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_STRING_H
