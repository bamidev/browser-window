#ifndef BW_APPLICATION_GTK_H
#define BW_APPLICATION_GTK_H

#include <gtk/gtk.h>
#include <pthread.h>
#include <stdbool.h>



typedef struct {
	GtkApplication* handle;
	int argc;
	char** argv;
	int exit_code;
	bool is_running;
	pthread_mutex_t is_running_mtx;
	pthread_t thread_id;
} bw_ApplicationImpl;



#endif//BW_APPLICATION_GTK_H