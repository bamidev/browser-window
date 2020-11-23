#ifndef BW_COMMON_H
#define BW_COMMON_H

#ifdef __cplusplus
extern "C" {
#endif



#include "assert.h"
#include "debug.h"
#if defined(BW_GTK)
#include "posix.h"
#elif defined(BW_WIN32)
#include "win32.h"
#endif

#include <stdint.h>



#define UNUSED( X ) \
	(void)( X );


typedef struct {
    uint16_t width;
    uint16_t height;
} bw_Dims2D;

typedef struct {
    uint16_t x;
    uint16_t y;
} bw_Pos2D;



#ifdef __cplusplus
} // extern "C"
#endif

#endif//BW_COMMON_H
