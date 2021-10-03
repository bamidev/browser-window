#include "err.h"

#include <string.h>
#include <stdio.h>


BW_ERR_MSG_DEF( bw_Err_msg_success, "success" )

char* bw_Err_message(const bw_Err* error) {
	return error->alloc_message(error->code, error->data);
}

char* bw_Err_msg_string( bw_ErrCode code, const void* message ) {
	(void)(code);

	return (char*)message;
}



void bw_Err_free( bw_Err* err ) {
	free( (void*)err->data );
}

bw_Err bw_Err_new_with_msg( bw_ErrCode code, const char* msg ) {

	size_t size = strlen( msg ) + 1;

	void* buffer = malloc( size );
	memcpy( buffer, msg, size );

	bw_Err e = {
		code,
		buffer,
		bw_Err_msg_string
	};
	return e;
}
