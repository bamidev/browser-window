# Browser Window

Browser Window is a simple Rust crate for using browser windows.
Browser Window was born from the lack of a good [Electron](https://www.electronjs.org/) alternative for Rust.
There a few very basic ones out there.
However, a few key features are missing, such as using them in asynchronous environments.
This is an attempt at creating an API that can be used to create:
* A graphical user interface with HTML/CSS/JS technology
* An authentication dialog for an external website that doesn't provide a (public) API for it
* An integrated web browser

This API is designed with asynchronous programming in mind,
giving the developer(s) the ability to use the API within an async/await context, but not forcing them to.
Using it with Rust's most common asynchronous runtime [Tokio](https://tokio.rs/), makes it so much more easier to use than having to deal with callbacks yourself.
You can view the [example](https://github.com/bamilab/browser-window/tree/master/example) to see how a graphical user interface within an asychronous context can be made.

## Requirements

Browser Window is built to support multiple underlying web engines, and is supposed to support multiple platforms.
However, at this stage, only [CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home) is a browser engine that is completely implemented, and only Windows is supported at the moment.

### CEF3

CEF3 is most easily obtained using [vcpkg](https://docs.microsoft.com/en-us/cpp/build/vcpkg?view=vs-2019).
After having downloaded and installed CEF3, append the CEF directory path to your PATH environment variable.
Then copy all files from the build directory (named Release or Debug) into your executable's working directory.

However, CEF3 doesn't only depend on their DLL files being available to the executable, they also need resource files to be located at the executables working directory.
Otherwise, you get errors like this:

[1023/133903.461:ERROR:icu_util.cc(247)] Couldn't mmap icu data file

So you need to copy everything from CEF's Resource dir into the executable's dir as wel.

So if you want to run the example, put everything from CEF's Release and Resource dir, into example/target/debug, and then it should work fine.


## License

This software is made available open source under a MIT license, for maximum freedom.

## Note

This project is currently in alpha phase. It is not really complete at this moment, and no versioning is done.
