# Getting Started

_BrowserWindow_ needs to be build with one of the following browser frameworks: [CEF](https://bitbucket.org/chromiumembedded/cef/wiki/Home) or [WebkitGTK](https://www.webkit.org/). They require either the `cef` or `webkitgtk` feature to be set.

_Browser Window_ currently relies on [CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home).
You will also need [cmake](https://cmake.org/) to set up CEF.
And on Windows, you will also need _Visual Studio_.

If you want to set up CEF by building it from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md).
However, it will take a lot of effort, time, memory & disk space for the compilation process.

## Picking the right browser framework

Here are the pros and cons of each browser framework. Choose wisely:

### CEF

*Pros:*
* Is available on all major platforms: Windows, MacOS, Linux (although MacOS support in _BrowserWindow_ needs some work).
If you want the exact same behavior of your app on all platforms, CEF is recommended.
* The cookie API of _BrowserWindow_ is supported.
* Is the only framework option which can be decently cross-compiled to Windows.

*Cons:*
* Can be a pain to set up correctly; requires a lot of files to be present for the executable, and needs the sandbox to have specific permissions.
* No option to link statically & generally not available in package managers, which forces you to
ship the shared libraries with your application.

### WebkitGTK

*Pros:*
* Generally easily installed on anything but Windows; a lot of distros have a package for it. There
is even a homebrew package for it on MacOS.

*Cons:*
* Compiling WebkitGTK and GTK for Windows is not supported.
* Static linking is not really supported for GTK.

### Edge WebView2

*Pro:*
* Preinstalled on Windows 11
* Can be statically linked to when using the `*-pc-windows-msvc` toolchain.

*Cons:*
* Currenty not yet working on _BrowserWindow_.
* Not cross-platform at all.
* The framework is not open source. Might be problematic for those concerned about privacy.

## Set up Bindgen

_BrowserWindow_ uses Bindgen, which needs some things to be set up on your system.
This is explained [here](https://rust-lang.github.io/rust-bindgen/requirements.html) pretty well.

## Set up WebkitGTK

If you're going to use WebkitGTK, a lot of systems have a convenient package for this. If not, just
make sure that `pkg-config` is set up to find all the headers & binaries.

### Debian APT

`apt install libwebkit2gtk-4.1-dev`

## Set up CEF

Keep in mind when you're going to use CEF, that _BrowserWindow_ is written to work for a specific version of CEF, and CEF does release new major versions fairly often. Therefore, it is recommended to
obtain the version that _BrowserWindow_ supports, which currently is v121. Use other versions at
your own risk.

CEF isn't generally available in package managers, so it needs to be set up manually. Luckily, there are binaries avaiable. You can also build it from source, but that is a whole other beast and it is
not covered by this guide.

### Download & Extract

You can get the latest prebuilt binaries [here](https://cef-builds.spotifycdn.com/index.html).
The 'minimal' package will be fine.
Once downloaded, you will need to extract it somewhere.

### Compilation

The library itself is already compiled for the binary distribution. However, there is a static 'wrapper' library that still needs to be built.
To do this, first run _cmake_ by running this on the command line from within the extracted folder:
```
cmake -DCMAKE_BUILD_TYPE=Debug .
```
Keep in mind that currently, it seems that the CEF wrapper library misses some symbols in release mode. This can cause some linker errors when trying to compile against the Release binaries.

*Note:* On Windows, you need to run it in a Visual Studio Developer Command Prompt, the regular
won't work. Also, use the `-A x64` option if you intend to build it for 64bit Windows.

#### Unix-like Systems

After you have run `cmake`, you can just simply run `make`. This will build the wrapper lib for CEF.

#### Windows

A newly generated Visual Studio solution has been generated in the folder.
You should build this solution's Release target with [Visual Studio](https://visualstudio.microsoft.com/vs/).
However, before you do, you need to change one setting in the project's settings.

Goto Project -> Properties -> Configuration Properties -> C/C++ -> Code Generation

If `Runtime Library` is set to `Multi-threaded (/MT)`, set it to `Multi-threaded DLL (/MD)`.
Rust links against the C runtime dynamically, and thus requires CEF to link to it dynamically as well.

Now you can build the solution.

### Environment Variables & Resource Files

Once you have extracted and compiled everything, we need to let _Browser Window_ know where it can find the header and library files to link to.
If you set environment variable `CEF_PATH` to the directory that you have extracted, Browser Window is able to find them.
Otherwise, `cargo` will fail to build `browser-window`.

You also need to copy resource and library files (`.so`/`.dll` and `.bin`), so that they are available to the executable.
Running `setup-cef-files.sh` or `setup-cef-files.bat` copies them to the target
folder so that you can just run your compiled binaries normally.
It will also change some permissions that CEF requires you to do.
`setup-cef-files.sh` will use `sudo`, so if you don't have `sudo`, inspect the file and execute the commands yourself.

For the library files, you could also just add the `Release` folder to your `PATH` environment variable for the `.so`/`.dll` files.

That's it!
Running `cargo run` should just work now.

If you encounter any issues, take a look at the [issue diagnosis page](https://github.com/bamilab/browser-window/blob/master/docs/ISSUE-DIAGNOSIS.md).
If that doesn't help, you can submit an issue to the repository.