#ifndef BW_GL_SURFACE_IMPL_H
#define BW_GL_SURFACE_IMPL_H

#include "../gl_surface.h"
#include "../window.h"



void bw_GlSurfaceImpl_destroy( bw_Window* window );

bw_GlSurfaceImpl bw_GlSurfaceImpl_new( bw_Window* window );



#endif//BW_GL_SURFACE_IMPL_H