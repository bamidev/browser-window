# Issue Diagnosis

If you are encountering issues while using and/or compiling _Browser Window_, this page is here to help you fix them.
If your issues isn't explained in here, please file the issue [here](https://github.com/bamilab/browser-window/issues).

## Compilation Errors

### Unix-like Systems

#### Missing gtk/gtk.h

TODO: Paste console error here...

This error means that the GTK development files are not available.
You should install them.

#### Missing "stdlib.h" or "stddef.h"

```rustc
./src/err.h:8:10: fatal error: 'stdlib.h' file not found
./src/err.h:8:10: fatal error: 'stdlib.h' file not found, err: true
thread 'main' panicked at 'Unable to generate FFI bindings! ...
```

`bindgen` might fail when `clang` isn't properly installed.
To fix this for Debian-like systems, try `apt install libclang-dev`.
To fix this on Windows, install clang [here]().

### Windows

#### Missing "Windows.h"

```rustc
src/application/win32.h:18:10: fatal error: 'Windows.h' file not found
```

This error occurs when the MSVC compiler can't find the Windows SDK headers.
This could mean that you didn't install the Windows SDK.
If you did, it could also mean that MSVC environment variables aren't set properly.
This tends to happen when using `bindgen` (see feature `bindgen`).

To fix this, call `cargo` from within the 'Cross Tools Command Prompt for VS 20**'.

If you want to work with another type of console, like that of Msys2 for example, you can do to following.
Run `vcvarsall.bat` (usually located in `C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build`) from the command line with your architecture and SDK version provided as arguments. Run it without arguments for more information.
When this is done, your console should be able to build `browser-window-ffi`.
But this needs to be done for every console that you want to build _Browser Window_ with, so this is not actually a long-term solution.

You can also not enable the `bindgen` feature, which prevents these issues in the first place.

#### error LNK2038

```rustc
libcpmt.lib(winapisupp.obj) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MT_StaticRelease' doesn't match value 'MD_DynamicRelease' in libbrowser_window_ffi-d71871b45632c28d.rlib(cef.o)
LINK : warning LNK4098: defaultlib 'LIBCMT' conflicts with use of other libs; use /NODEFAULTLIB:library
```

This happends when you didn't follow the getting started guide properly.
You need to built CEF with the `/MD` flag.

### Other C++ Errors

Sometimes, you may run into C++ compilation errors. Generally speaking, this should not happen for versions of _Browser Window_ that where published.
However, _Browser Window_ still depends on one external C++ API, _CEF_.
So, new versions of _CEF_ may break compatability with _Browser Window_.
To fix this, simply look at the _latest version known to work_ specified in the [getting started guide](./GETTING-STARTED.md).

## Runtime Errors

TODO