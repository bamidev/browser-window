# Getting Started on anything other than Windows

*Warning:* Currently support for Unix-like systems is on the way.
However, this guide is not tried and tested yet.

There are basically two dependencies.
_Browser Window_ depends on GTK 3 and CEF 3 for all non-Windows systems.
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
First of all, to get CEF with GTK support, we need to build it manually.

## Building the source code

TODO

## Building the wrapper lib

To link with CEF, there is a static wrapper lib and a large shared (.so) lib file.
The shared library file is already precompiled, but the static wrapper lib isn't.
The build it you need `cmake`.

Run from within the extracted folder:
```
cd cef_binary_*
cmake .
make
```
When done, the file is located at `./libcef_dll_wrapper/libcef_dll_wrapper.a`.

## Resources

Then we need to tell our compiler & linker where they can find our library files by setting `CEF_PATH` to the directory we've just extracted:
```
export CEF_PATH=/my/path/to/cef_binary_ ... _minimal
```

### Resource Files
To last thing that needs to be done is that some files need to be made available to the executable.
Copy all .bin files from the Release folder to the executables directory.
If you are running your crate with `cargo run`, your executable is located at `./target/debug`.
Also copy all files from the Resource folder to the same directory.

### Library Files
If you want to be able to run your crate, make sure to point `LD_LIBRARY_PATH` (or `DYLD_FALLBACK_LIBRARY_PATH` for MacOS) to the Release folder as well.
Either that, or copy the .so files from that folder to the executable directory as well.

### Sandbox File
Make sure that the file `chrome-sandbox` in `CEF_PATH` or the executable directory has file mode `4755` and is owned by `root:root`.
```sh
chown root:root chrome-sandbox
chmod 4755 chrome-sandbox
```

That's it!
A call to `cargo run` will do it.

## Building From Source

If you really want to build CEF from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/BranchesAndBuilding.md#markdown-header-automated-method).
You still need to set up the environment variables and copy the files into the working directory afterwards.
