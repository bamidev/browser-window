#ifndef BW_ASSERT_H
#define BW_ASSERT_H

#ifdef __cplusplus
extern "C" {
#endif

#include <assert.h>
#include <stdio.h>

#ifdef __cplusplus
} //extern "C"
#endif



// Asserts if CONDITION evaluates to false.
// MESSAGE will be printed in standard error output.
// The same arguments provided to fprintf can be provided in this macro, like this:
//     BW_ASSERT( false, "Unable to find number %i", my_number )
#define BW_ASSERT( CONDITION, ... ) \
	if ( !(CONDITION) ) { \
		fprintf( stderr, "[ASSERTION %s:%i] ", __FILE__, __LINE__ ); \
		fprintf( stderr, __VA_ARGS__ ); \
		fprintf( stderr, "\n" ); \
		assert( CONDITION ); \
	}

#define BW_PANIC( ... ) \
	BW_ASSERT( 0, __VA_ARGS__ )



#endif//BW_ASSERT_H
