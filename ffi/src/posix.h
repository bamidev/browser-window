#ifndef BW_POSIX_H
#define BW_POSIX_H

#include "assert.h"

#include <errno.h>



#define BW_POSIX_ASSERT_SUCCESS( ERRNO ) \
    BW_ASSERT( ERRNO == 0, "[posix error %i] %s", ERRNO, strerror( ERRNO ) )



#endif//BW_POSIX_H