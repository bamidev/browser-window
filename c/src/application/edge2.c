#include "edge2.h"
#include "impl.h"
#include "../common.h"

#include <windows.h>


void bw_ApplicationEngineImpl_free(bw_ApplicationEngineImpl* app) {
    UNUSED(app);
}

bw_Err bw_ApplicationEngineImpl_initialize( bw_ApplicationEngineImpl* impl, bw_Application* app, int argc, char** argv, const bw_ApplicationSettings* settings ) {
    CoInitializeEx(NULL, COINIT_APARTMENTTHREADED);
    BW_ERR_RETURN_SUCCESS;
}