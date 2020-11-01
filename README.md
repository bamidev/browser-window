# Browser Window

Browser Window is a simple Rust crate for using browser windows.
Browser Window was born from the lack of a good [Electron](https://www.electronjs.org/) alternative for Rust.
There a few very basic ones out there.
However, a few key features are missing, such as using them in asynchronous environments.
This is an attempt at creating an API that can be used to build a graphical user interface with HTML/CSS/JS technology, or just to have browser functionality within your application.
Another feature that will be made available in the future is the ability to obtain HTTP cookies.
This allows you to create authentication dialogs for websites that don't provide public API's.

This API is designed with asynchronous programming in mind from the ground up.
This gives the developer(s) the ability to use this API within an async/await context, but not forcing them to.
This makes it so much easier, than having to deal with callbacks.
You can view the [example](https://github.com/bamilab/browser-window/tree/master/example) to see how a graphical user interface within a asynchronous context could be made.

## Requirements

Browser Window currently relies on [CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home) as its browser engine.
Also, only windows is the supported platform at the moment.

### CEF3

CEF3 is most easily obtained using [vcpkg](https://docs.microsoft.com/en-us/cpp/build/vcpkg?view=vs-2019).
After having downloaded (and if necessary compiled) CEF3, append the CEF directory path to your PATH environment variable.
Then copy all files from the build directory (named Release or Debug) into your executable's working directory.

However, CEF3 doesn't only depend on their DLL files being available to the executable, they also need resource files to be located at the executables working directory.
Otherwise, you get errors like this:

[1023/133903.461:ERROR:icu_util.cc(247)] Couldn't mmap icu data file

So you need to copy everything from CEF's Resource dir into the executable's dir as wel.

If you want to run the example, put everything from CEF's Release and Resource dir, into example/target/debug, and then it should work fine with "cargo run".


## License

This software is made available open source under a MIT license, for maximum freedom.

## Development

If you want to help out, you're more than welcome! I also need someone to implement support for iOS with WebKit.

## Comming Soon

This project is very new. At the moment only very basic functionality is made available. More will come soon.

* Linux support
* Cookies (including HttpOnly cookies)
