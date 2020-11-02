#include "../application.h"

#include <winrt/Windows.Foundation.h>

#pragma comment(lib, "windowsapp.lib")



void bw_Application_init( bw_Application* app ) {
	init_apartment(winrt::apartment_type::single_threaded);
}

void bw_Application_uninit( bw_Application* app ) {}
