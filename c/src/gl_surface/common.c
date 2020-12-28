#include "../gl_surface.h"
#include "impl.h"



void bw_GlSurface_destroy( bw_GlSurface* surface ) {
	bw_GlSurfaceImpl_destroy( &surface->impl );
}

bw_GlSurface* bw_GlSurface_new( bw_Window* window ) {
	bw_GlSurface* surface = (bw_GlSurface*)malloc( sizeof( bw_GlSurface ) );

	surface->impl = bw_GlSurfaceImpl_new( window );

	return surface;
}