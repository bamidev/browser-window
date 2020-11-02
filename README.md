# Browser Window

Browser Window is a simple Rust crate for using browser windows.
This can be used to build a graphical user interface with HTML/CSS/JS technology, or just to have browser functionality within your application.
Browser Window was born from the lack of a good [Electron](https://www.electronjs.org/) alternative for Rust.
There are actually a few ones out there already.
However, they seem to be lacking a few important things.

## The problem Browser Window is solving

The main problem is that they use browser engine's already available on the system (except for Linux).
This is useful if you want to ship your application as a single executable file.
However, on Windows it uses the old Edge engine, which is not being maintained anymore.
Moreover, some other simple features are not available, like for example getting the result back from a JavaScript evaluation.

This API is designed with asynchronous programming in mind, from the ground up.
The developer(s) have the ability to use this API within an async/await environment, but it does not force them to.
It makes it so much easier than having to deal with callbacks in your code.
You can view the [example](https://github.com/bamilab/browser-window/tree/master/example) to see how easily a graphical user interface can be made, with asynchronous/multi-threaded programming.

## Requirements

At this moment, Windows is the only supported platform, but other platforms will follow.

Browser Window currently has two options for a browser engine.
[CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home) is the default.
However, if you want minimal setup effort, or want to ship a single independent executable, you can use the old Edge browser engine. You can do this by enabling the feature "edge" in your Cargo.toml file.
If you want to have the newest of web technologies available to you, use CEF3.

### CEF3

CEF3 is most easily obtained using [vcpkg](https://docs.microsoft.com/en-us/cpp/build/vcpkg?view=vs-2019).
Otherwise, you can also find prebuilt binaries [here](http://opensource.spotify.com/cefbuilds/index.html), or [build it yourself](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md).
After having downloaded (and if necessary compiled) CEF3, append the CEF directory path to your PATH environment variable.
Then copy all files from the build directory (named Release or Debug) into your executable's working directory.

However, CEF3 doesn't only depend on their DLL files being available to the executable, they also need resource files to be located at the executables working directory.
Otherwise, you get errors like this:

[1023/133903.461:ERROR:icu_util.cc(247)] Couldn't mmap icu data file

So you need to copy everything from CEF's Resource dir into the executable's dir as wel.

If you want to run the example, put everything from CEF's Release and Resource dir, into example/target/debug, and then it should work fine with "cargo run".

### Edge

The (old) Edge engine (a.k.a. EdgeHTML) is available in the [Windows 10 SDK](https://developer.microsoft.com/en-US/windows/downloads/windows-10-sdk/).
This SDK is usually automatically installed on your Windows 10 installation.
The only thing you need to do it enable feature "edge" in this crate, if you want to enable this engine.

## License

This software is available as open source software under a MIT license, for maximum freedom.

## Development

If you want to help out, you're more than welcome! I could use some help with implementing iOS support.

## Comming Soon

This project is very new. At the moment only very basic functionality is available. There will be more soon:

* Linux support
* Cookie support (including HttpOnly cookies)
