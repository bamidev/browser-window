#ifndef BW_WINDOW_WIN32_H
#define BW_WINDOW_WIN32_H

#include "../window.h"

#define WIN32_LEAN_AND_MEAN
#include <Windows.h>



struct bw_WindowDispatchData {
	bw_WindowDispatchFn func;
	bw_Window* window;
	void* data;
};



LRESULT CALLBACK bw_Window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp);



#endif//BW_WINDOW_WIN32_H
