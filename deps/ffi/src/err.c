#include "err.h"

#include <string.h>



BW_ERR_MSG_DEF( bw_Err_msg_success, "success" )

char* bw_Err_msg_string( bw_ErrCode code, const void* message ) {
	return (char*)message;
}


void bw_Err_free( bw_Err* err ) {
	free( err->data );
	free( err );
}
