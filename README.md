# Browser Window

Browser Window is a simple Rust crate for managing browser windows.
You can build graphical user interfaces with HTML/CSS/JS technology, or just use browser functionality in your application.
Browser Window was born from the lack of a good [Electron](https://www.electronjs.org/) alternative for Rust.
There are actually a few ones out there already.
However, they are lacking a few important things.

## The problems Browser Window is solving

The main problem is that they use browser engine's already available on the system (except for Linux).
This is useful if you want to ship your application as a single executable file.
However, using different engines on each platform makes it hard to make your application work consistently on each platform, especially when using newer features that are not supported by each browser engine yet.
Using one engine for each platform is best, even though you need to ship the engine together with your application.

Another problem you run into when using the browser engines already available on the system, is that in the case of Windows, that engine is not being maintained anymore.
Windows has the EdgeHTML engine that Edge used before it switched to the CEF engine.
If you want to use newer HTML5/CSS3 features, you can run into problems.

Moreover, some other simple features are currently not available in other Rust crates, that could make life so much easier for the developer(s).
Like for example, getting the resulting value back from a JavaScript evaluation.

This API is designed with asynchronous programming in mind, from the ground up.
The developer(s) have the ability to use this API within an async/await environment without much hassle.
Single-threaded applications are also possible though.
You can view the [example](https://github.com/bamilab/browser-window/tree/master/example) to see how easily a GUI with Browser Window can be made, within a multi-threaded environment.

## Comparison with Electron

Electron is a pretty complete library.
This is nice, but Browser Window aims to adhere to the Unix philosophy.
This API will be kept (very) simple.
Other functionality like notifications, tray icons will not be available in this crate.
Other cross-platform crates already exist for this.


## Requirements

At this moment, Windows is the only supported platform, but support for other platforms will follow.

Browser Window currently has support for two different browser engines.
[CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home) is the default, but you need to set it up before you can use Browser Window.

However, if you want minimal setup effort, or want to ship a single independent executable to the end user, you can use the old Edge browser engine as well.
You can do this by enabling the feature "edge" in your Cargo.toml file.
But if you want to have the newest of web technologies at your hands, use CEF3.

### CEF3

CEF3 is most easily obtained using [vcpkg](https://docs.microsoft.com/en-us/cpp/build/vcpkg?view=vs-2019).
Otherwise, you can also get pre-built binaries [here](http://opensource.spotify.com/cefbuilds/index.html), or [build it yourself](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md).
After having downloaded (and if necessary compiled) CEF3, append the CEF directory path to your PATH environment variable.
Then copy all files from the build directory (named Release or Debug) into your executable's working directory.

CEF3 also needs resource files in the working directory.
All files and folders in Resource need to be located there.
Otherwise, you might get an error like this:

[1023/133903.461:ERROR:icu_util.cc(247)] Couldn't mmap icu data file

If you want to run the example with CEF3 enabled, just put everything from CEF's Release and Resource dir, into example/target/debug, and then it should work fine with "cargo run".

### Edge

The (old) EdgeHTML engine is available in the [Windows 10 SDK](https://developer.microsoft.com/en-US/windows/downloads/windows-10-sdk/).
This SDK is usually automatically installed on your Windows 10 system.
The only thing you need to do it enable feature "edge" in this crate, if you want to enable this engine.

## License

This software is available as open source software under a MIT license, for maximum freedom.

## Development

If you want to help out, you're more than welcome! I could use some help with implementing iOS support actually.

## Comming Soon

At the moment, basic functionality is available, but there is more to come.
These are the features that are awaiting implementation:

* Linux support (through GTK)
* iOS support (through Cocoa)
* Cookie support (including HttpOnly cookies)
* Utilities for packaging your app for distribution
