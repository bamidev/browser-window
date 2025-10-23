#include "../common.h"

#include "impl.h"


void bw_ApplicationEngineImpl_free(bw_ApplicationEngineImpl* impl) { UNUSED(impl); }

bw_Err bw_ApplicationEngineImpl_initialize( bw_ApplicationEngineImpl* impl, bw_Application* app, int argc, char** argv, const bw_ApplicationSettings* settings ) {
	UNUSED(impl);
	UNUSED(app);
	UNUSED(argc);
	UNUSED(argv);
	UNUSED(settings);
	BW_ERR_RETURN_SUCCESS;
}
