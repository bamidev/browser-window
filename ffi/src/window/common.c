#include "../window.h"



const bw_Application* bw_Window_get_app( bw_Window* window ) {
	return window->app;
}

bool bw_Window_is_closed( const bw_Window* window ) {
	return window->closed;
}
