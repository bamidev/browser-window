# Issues that need fixing:

### JavaScript result callback might not be called from the correct thread.

Currently CEF is implemented by spawning its own message loop in its own thread.
This is nice, but currently the JavaScript result callback is being called from there.
When `bw_BrowserWindow_evalJs` is called, the callback needs to be called from the GUI thread.
This can be done by dispatching the callback, and then calling it from there.

### GTK not tested yet

The GTK implementation needs testing.
