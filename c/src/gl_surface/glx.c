#include "../gl_surface.h"



bw_GlSurfaceImpl bw_GlSurfaceImpl_new( bw_Window* window ) {

	glxCreateWindow( display, config, bw_Window_getXWindowHandle( window ) )
}