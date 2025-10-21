#ifndef BW_WINDOW_IMPL_H
#define BW_WINDOW_IMPL_H

#ifdef __cplusplus
extern "C" {
#endif


bw_WindowImpl bw_WindowImpl_new(
	bw_Window* window,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options
);

void bw_WindowImpl_close( bw_WindowImpl* window );
void bw_WindowImpl_hide( bw_WindowImpl* window );
void* bw_WindowImpl_innerHandle(bw_WindowImpl* window);
void bw_WindowImpl_show( bw_WindowImpl* window );


#ifdef __cplusplus
}
#endif

#endif//BW_WINDOW_IMPL_H
