#include "../window.h"

#include "../common.h"



gboolean _bw_WindowGtk_closeHandler( GtkWidget* handle, gpointer data );
gboolean _bw_WindowGtk_stateHandler( GtkWidget *widget, GdkEventWindowState *event, gpointer user_data );



bw_WindowImpl bw_WindowImpl_new(
	const bw_Window* window,
	bw_CStrSlice _title,
	int width, int height,
	const bw_WindowOptions* options
) {
	GtkWidget* gtk_handle = gtk_application_window_new( window->app->impl.handle );

	gtk_widget_hide_on_delete( gtk_handle );

	if ( window->parent != 0 ) {
		gtk_widget_set_parent_window( gtk_handle, GDK_WINDOW( window->parent->impl.handle ) );
		gtk_window_set_destroy_with_parent( GTK_WINDOW(gtk_handle), FALSE );
	}

	// Title
	// TODO: Use g_locale_from_utf8 to get valid string
	gchar* title = bw_string_copyAsNewCstr( _title );
	gtk_window_set_title( GTK_WINDOW(gtk_handle), title );
	free( title );

	// Width and height
	gtk_window_resize( GTK_WINDOW(gtk_handle), width, height );
	gtk_window_set_resizable( GTK_WINDOW(gtk_handle), options->resizable );

	// If both not minimizable and not resizable, make it a dialog
	gtk_window_set_type_hint( GTK_WINDOW(gtk_handle), GDK_WINDOW_TYPE_HINT_DIALOG );

	g_signal_connect( gtk_handle, "window-state-event", G_CALLBACK( _bw_WindowGtk_stateHandler ), (gpointer)window );
	//g_signal_connect( gtk_handle, "destroy-event", G_CALLBACK( _bw_WindowGtk_closeHandler ), (gpointer)window );

	gtk_widget_show_all( gtk_handle );

	bw_WindowImpl impl;
	impl.handle = gtk_handle;

	return impl;
}

void bw_WindowImpl_destroy( bw_Window* window ) {
	gdk_window_destroy( GDK_WINDOW(window->impl.handle) );
}

void bw_WindowImpl_hide( bw_Window* window ) {
	gdk_window_hide( GDK_WINDOW(window->impl.handle) );
}


gboolean _bw_WindowGtk_stateHandler( GtkWidget *widget, GdkEventWindowState *event, gpointer user_data ) {
	UNUSED(user_data);

	bw_Window* window = (bw_Window*)user_data;

	// If iconified
	if ( event->new_window_state & GDK_WINDOW_STATE_ICONIFIED ) {

		// If window is not minimizable, deiconify
		if ( !window->impl.minimizable ) {
			return FALSE;
			gtk_window_deiconify( GTK_WINDOW( widget ) );
		}
	}

	return TRUE;
}

gboolean _bw_WindowGtk_closeHandler( GtkWidget* handle, gpointer data ) {
	UNUSED(handle);

	bw_Window* window = (bw_Window*)data;

	bw_WindowImpl_hide( window );
	return FALSE;
}
