#ifndef BW_APPLICATION_CEF_WINDOW_H
#define BW_APPLICATION_CEF_WINDOW_H

#if !defined(BW_CEF)
#error "BW_CEF needs to be defined in order to used BW_CEF_WINDOW!"
#endif

#ifdef __cplusplus
extern "C" {
#endif



typedef struct {
	int exit_code;
	bool is_running;
} bw_ApplicationImpl;



#ifdef __cplusplus
}
#endif

#endif//BW_APPLICATION_CEF_WINDOW_H
