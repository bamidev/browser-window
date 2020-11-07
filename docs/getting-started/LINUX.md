# Getting Started on Linux

There are basically two dependencies.
Browser Window depends on GTK 3 and CEF 3.
Also, `pkg-config` is required.

##### GTK

GTK 3 is generally easily available on most Linux distros.
You could use the following commands (as root) to install the development files:

###### For Debian & Ubuntu
`apt install pkg-config libgtk-3-dev`

In any case, just make sure that `pkg-config gtk+-3.0 --cflags` works.

##### CEF

CEF takes a bit more effort to have it set up properly.

###### Download & Extract

The easiest and quickest way to set up CEF is to get the binary distribution.
This means you don't have to build the whole project, which is enormous and takes at least several hours and a huge amount of disk space.
You can get the latest prebuilt binaries [here](http://opensource.spotify.com/cefbuilds/index.html).
The minimal version will be fine.

##### Building the wrapper lib

To link with CEF, there is a static wrapper lib and a large shared (.so) lib file.
The shared library file is already precompiled, but the static wrapper lib isn't.
The build it you need `cmake`.

Run `cmake .` within the extracted folder.

When done, the file is located at `./libcef_dll_wrapper/libcef_dll_wrapper.a`.

##### Environment Variables & Resource Files

The only thing that is left to do, is to tell our compiler & linker where they can find our library files.

Once you have extracted everything, we need to set up some environment variables, so that the MSVC compiler knows where it can find the header and library files.
You can find your system environment variables by going to: Control Panel > System and Security > System > Advanced system settings.
Then click on the "Environment variables..." button.

* Add the extracted folder to the `$CPATH` variable: `export CPATH=$CPATH:mydir`.
* Add the Release folder inside that extracted folder to your %LIB% variable.
* Do one of the following:
    1. Add the same Release folder to your `%PATH%` variable.
    2. Copy all .dll files in the Release folder to the executables working directory.
       This is target/debug or target/release within your crate's folder.
* Copy all .bin files from the Release folder, into the working directory.
* Copy all files from the Resource folder, into the working directory.

That's it!
A call to `cargo run` will do it.

##### Building From Source

If you really want to build CEF from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md#markdown-header-windows-setup).
You still need to set up the environment variables and copy the files into the working directory afterwards.
