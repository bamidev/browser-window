#ifndef BW_APPLICATION_GTK_H
#define BW_APPLICATION_GTK_H

#include <gtk/gtk.h>



typedef struct {
	GtkApplication* handle;
    int argc;
    char** argv;
	int exit_code;
} bw_ApplicationImpl;

struct bw_ApplicationDispatchData {
	struct bw_Application* app;
	bw_ApplicationDispatchFn func;
	void* data;
};



#endif//BW_APPLICATION_GTK_H
