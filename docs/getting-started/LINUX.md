# Getting Started on Linux

There are basically two dependencies.
Browser Window depends on GTK 3 and CEF 3 for all non-Windows systems.
Also, `pkg-config` and `cmake` are required.

## GTK

GTK 3 is generally easily available on most Linux distros.
You could use the following commands (as root) to install the development files:

###### On Debian & Ubuntu
`apt install pkg-config libgtk-3-dev`

###### On Arch
`pacman -S gtk3`

In any case, just make sure that `pkg-config gtk+-3.0 --cflags` works.

## CEF

CEF takes a bit more effort to have it set up properly.

## Download & Extract

The easiest and quickest way to set up CEF is to get the binary distribution.
Building the source code is very, very time consuming.
You can get the latest prebuilt binaries [here](http://opensource.spotify.com/cefbuilds/index.html).
The minimal version will be fine.

You need to extract this archive:
`tar -xvf cef_binary_*_minimal.tar.bz`

## Building the wrapper lib

To link with CEF, there is a static wrapper lib and a large shared (.so) lib file.
The shared library file is already precompiled, but the static wrapper lib isn't.
The build it you need `cmake`.

Run from within the extracted folder:
```
cd cef_binary_*
cmake .
```
When done, the file is located at `./libcef_dll_wrapper/libcef_dll_wrapper.a`.

## Environment Variables & Resource Files

Then we need to tell our compiler & linker where they can find our library files by setting `CEF_PATH` to the directory we've just extracted:
```export CEF_PATH=/my/path/to/cef_binary_ ... _minimal```

To last thing that needs to be done is that some files need to be made available to the executable.
Copy all .dll and .bin files to the working directory, or add the Release folder to your `$PATH` environment variable.

That's it!
A call to `cargo run` will do it.

## Building From Source

If you really want to build CEF from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md#markdown-header-linux-setup).
You still need to set up the environment variables and copy the files into the working directory afterwards.
