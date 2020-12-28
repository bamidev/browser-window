/*	This file contains all functionality to make tool-less debugging a little bit easier.
 *	Most macros are simply shorter ways to print stuff to the console, that only get compiled in debug builds.
 */
#ifndef BW_DEBUG_H
#define BW_DEBUG_H

#ifdef __cplusplus
extern "C" {
#endif



#include <stdio.h>



// Print MESSAGE to the standard error output.
// Additional information can be provided into the message, just like printf.
#ifndef NDEBUG
#define BW_DEBUG( ... ) \
{ \
	fprintf( stderr, "[DEBUG %s:%d] ", __FILE__, __LINE__ ); \
	fprintf( stderr, __VA_ARGS__ ); \
	fprintf( stderr, "\n" ); \
}
#else
#define BW_DEBUG( ... )
#endif



#ifdef __cplusplus
} //extern "C"
#endif



#endif//BW_DEBUG_H
