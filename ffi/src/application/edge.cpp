#include "../application.h"
#include "../common.h"

#include "impl.h"

#include <winrt/Windows.Foundation.h>

#pragma comment(lib, "windowsapp.lib")



void bw_ApplicationEngineImpl_finish( bw_ApplicationEngineImpl* app ) {
	UNUSED( app );
}

bw_ApplicationEngineImpl bw_ApplicationEngineImpl_start( bw_Application* app, int argc, char** argv ) {
	UNUSED( app );
	UNUSED( argc );
	UNUSED( argv );

	init_apartment(winrt::apartment_type::single_threaded);

	bw_ApplicationEngineImpl impl;
	return impl;
}
