#ifndef BW_WIN32_WINDOW_H
#define BW_WIN32_WINDOW_H



#define WIN32_LEAN_AND_MEAN
#include <Windows.h>



typedef struct {
	HWND handle;
	DWORD style;
	BYTE opacity;
} bw_WindowImpl;



#endif//BW_WIN32_WINDOW_H
