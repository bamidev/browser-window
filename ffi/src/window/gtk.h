#ifndef BW_WINDOW_GTK_H
#define BW_WINDOW_GTK_H

#include <gtk/gtk.h>



typedef struct {
	GtkWidget* handle;
	gboolean minimizable;
} bw_WindowImpl;



#endif//BW_WINDOW_GTK_H
