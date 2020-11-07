#include "string.h"

#include <stdlib.h>
#include <string.h>



char* bw_string_copyAsNewCstr( bw_CStrSlice str ) {
	char* new_str = (char*)malloc( str.len + 1 );
	memcpy( new_str, str.data, str.len );
	new_str[ str.len ] = '\0';
	return new_str;
}

void bw_string_freeCstr( char* str ) {
	free( str );
}
