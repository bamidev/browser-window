#ifndef BW_GL_SURFACE_H
#define BW_GL_SURFACE_H

#include "window.h"



typedef struct {
	/// Implementation related data
	bw_GlSurfaceImpl impl;
} bw_GlSurface;



/// Removes and cleans up the GL surface, freeing it from memory as well.
void bw_GlSurface_destroy( bw_GlSurface* surface );

/// Creates a new GL surface inside the given window.
bw_GlSurface* bw_GlSurface_new( bw_Window* window );



#endif//BW_GL_SURFACE_H