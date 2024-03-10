# Getting Started

_BrowserWindow_ needs to be build with one of the following browser frameworks: [CEF](https://bitbucket.org/chromiumembedded/cef/wiki/Home), [WebkitGTK](https://www.webkit.org/) or [Edge WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/?form=MA13LH). They require either the `cef`, `webkitgtk` or `edge2` feature to be set.

## Picking the right browser framework

Here are the pros and cons of each browser framework. Choose wisely:

### CEF

*Pros:*
* Is available on all major platforms: Windows, MacOS, Linux (although MacOS support in _BrowserWindow_ needs some work).
If you want the exact same behavior of your app on all these platforms, CEF is recommended.
* Supports the cookie API.
* Supports the most event types.

*Cons:*
* Can be a pain to set up correctly; requires a lot of files to be present for the executable & compilation (especially on Windows) needs some extra care to get it done correctly. Although, there are scripts in this repository to do it for you.
* No option to link statically & generally not available through package managers.

### WebkitGTK

*Pros:*
* Generally easily installed on Unix-like systems; a lot of linux distros have a package for it.

*Cons:*
* Compiling WebkitGTK or GTK for Windows is not supported.
* Homebrew package for WebkitGTK seems to be unmaintained.
* Static linking is also not really supported for GTK.

### Edge WebView2

*Pro:*
* Preinstalled on Windows 11
* Can be statically linked to when using the `*-pc-windows-msvc` toolchain.
* Can be used to cross-compile for Windows.

*Cons:*
* Not cross-platform at all.
* The framework is not open source, which might be problematic for those concerned about privacy.

## Set up Bindgen

_BrowserWindow_ uses Bindgen, which needs some things to be set up on your system.
This is explained [here](https://rust-lang.github.io/rust-bindgen/requirements.html) pretty well.

## Set up WebkitGTK

If you're going to use WebkitGTK, a lot of systems have a convenient package for this. If not, just
make sure that `pkg-config` is set up to find all the headers & binaries.

### Debian APT

`apt install libwebkit2gtk-4.1-dev`

## Set up Edge WebView2

If you're going to use the Microsoft Edge WebView2 framework, you need to make sure that the runtime
is installed on your system.

### Some notes on cross-compilation

Cross compilation to the `*-pc-windows-gnu` target works. Just make sure that MinGW is set up to
find the headers of the win32 API.

Moreover, you need to ship the executable together with the WebView2Loader.dll file.
It can be obtained on non-Windows systems, by installing nuget, and downloading it with:

`nuget install Microsoft.Web.WebView2`

It will be installed in the current working directory, and then the .dll file can be located at
`Microsoft.Web.WebView2.*/build/native/<arch>`.

## Set up CEF

Keep in mind when you're going to use CEF, that _BrowserWindow_ is written to work for a specific version of CEF, and CEF does release new major versions fairly often. Therefore, it is recommended to
obtain the version that _BrowserWindow_ supports, which currently is v122. Use other versions at
your own risk.

CEF isn't generally available in package managers, so it needs to be set up manually. Luckily, there are binaries avaiable. You can also build it from source, but that is a whole other beast and it is
not covered by this guide.

If you want to set up CEF by building it from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md).
However, it will take a lot of effort, time, memory & disk space for the compilation process.

Otherwise, if you're on linux, here it the TL;DR version on setting up CEF:

#### Linux

```
git clone https://github.com/bamidev/browser-window
cd browser-window
./get-cef.sh                                   # Download & compile CEF
export CEF_PATH= ...                           # Set environment variable
./setup-cef-files.sh                           # Put necessary files in target/debug
cargo run --example terminal --features cef    # Run example code to test if it works
```

#### Windows

```
git clone https://github.com/bamidev/browser-window
cd browser-window    # Download & compile CEF
.\get-cef.ps1        # Set environment variable
```
Then, add the printed environment variable to your system environment variables for next time.
```
.\setup-cef-files.bat                          # Put necessary files in target/debug
cargo run --example terminal --features cef    # Run example code to test if it works
```

As long as `CEF_PATH` is set correctly on the system you're compiling on, compilation of `browser-window`
should work even if it is a dependency of your crate.
But you still need to copy files (just like `setup-cef-files.sh` does) to your crate's target/debug directory.

### Download & Extract

If you're going to set it up manually, you need to get the binaries first.
You can get the latest prebuilt binaries [here](https://cef-builds.spotifycdn.com/index.html).
The 'minimal' package will be fine.
Once downloaded, you will need to extract it somewhere.

### Compilation

The library itself is already compiled for the binary distribution. However, there is a static 'wrapper' library that still needs to be built.
To do this, first run _cmake_ by running this on the command line from within the extracted folder:
```
cmake .
```
Keep in mind that currently, right out of the box, CEF seems to miss a symbol when compiled.
This can be solved by defining `DCHECK_ALWAYS_ON` before compiling, which can be straightforward in
Visual Studio.
Otherwise, you can also just add `add_compile_definitions(DCHECK_ALWAYS_ON=1)` to the beginning of
the `CMakeLists.txt` file.
Another solution might be to build the cmake project in debug mode: `cmake -DCMAKE_BUILD_TYPE=Debug .`

#### Unix-like Systems

After you have run `cmake`, you can just simply run `make`. This will build the wrapper lib for CEF.

#### Windows

A newly generated Visual Studio solution has been generated in the folder.
You should build this solution's Release target with [Visual Studio](https://visualstudio.microsoft.com/vs/).
However, before you do, you need to change one setting in the project's settings.

Make sure to compile the project as a static lib (.lib).

Now you can just build the solution.

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