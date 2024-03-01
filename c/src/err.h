#ifndef BW_ERR_H
#define BW_ERR_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdlib.h>
#include <string.h>



typedef unsigned int bw_ErrCode;

typedef struct bw_Err {
	bw_ErrCode code;
	const void* data;
	char* (*alloc_message)( bw_ErrCode code, const void* data );	/// Returns a newly allocated pointer to a null terminated utf8 string that describes the error
} bw_Err;

#define BW_ERR_CODE_SUCCESS 0

#define BW_ERR_IS_FAIL(ERROR) \
	(ERROR).code != BW_ERR_CODE_SUCCESS
#define BW_ERR_IS_OK(ERROR) \
	(ERROR).code == BW_ERR_CODE_SUCCESS

#define BW_ERR_RETURN( CODE, DATA_PTR, MSG_FUNC ) \
	{ bw_Err r; r.code = CODE; r.data = (const void*)DATA_PTR; r.alloc_message = MSG_FUNC; return r; }

#define BW_ERR_DECLARE(VAR_NAME, CODE, DATA_PTR, MSG_FUNC) \
	bw_Err VAR_NAME = { CODE, DATA_PTR, MSG_FUNC };

#define BW_ERR_RETURN_SUCCESS \
	BW_ERR_RETURN( BW_ERR_CODE_SUCCESS, 0, bw_Err_msg_success )

#define BW_ERR_DECLARE_SUCCESS(VAR_NAME) \
	BW_ERR_DECLARE(VAR_NAME, BW_ERR_CODE_SUCCESS, 0, bw_Err_msg_success)

#define BW_ERR_MSG_DEF( FUNC, MSG ) \
	char* FUNC( bw_ErrCode code, const void* data ) { \
		(void)(code); \
		(void)(data); \
		char* msg = (char*)malloc( strlen( MSG ) + 1 ); \
		strcpy( msg, MSG ); \
		return msg; \
	}



char* bw_Err_msg_success( bw_ErrCode, const void* );
char* bw_Err_msg_string( bw_ErrCode, const void* );

// Should always be called on a bw_Err.
// Frees internal data from the heap.
// TODO: Rename to bw_Err_destroy
//       Now it sounds like its going to free the memory of bw_Err as well...
void bw_Err_free( bw_Err* err );
// Creates a new initialized bw_Err, with the given code and message.
// The alloc_message pointer will return the same message as given here.
bw_Err bw_Err_new_with_msg( bw_ErrCode code, const char* msg );

char* bw_Err_message(const bw_Err* error);



#ifdef __cplusplus
}	// extern "C"
#endif

#endif//BW_ERR_H
