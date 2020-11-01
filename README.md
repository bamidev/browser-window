# Browser Window

Browser Window is a simple Rust crate for using browser windows.
This can be used to build a graphical user interface with HTML/CSS/JS technology, or just to have browser functionality within your application.
Browser Window was born from the lack of a good [Electron](https://www.electronjs.org/) alternative for Rust.
There are actually a few ones out there already.
However, they seem to be lacking a few important things.

## The problem Browser Window is solving

The main problem is that they depend on crate web-view, which uses browser engine API's already available on the OS itself, like Edge on Windows.
This makes it really easy to build and ship applications because you don't really need to install and ship any libraries on Windows and iOS.
However, the Edge API that's preinstalled on Windows is not being maintained anymore.
Also, with Linux you still need to provide WebKit anyway, so basically only iOS could provide the promise of compiling to single executables while at the same time keeping up to date with the newest web technology.
Moreover, the Windows web-view implementation, at this time of writting, causes segfaults because there are some problems with memory management.
What also would be nice is a nice asynchronous interface.

This API is designed with asynchronous programming in mind, from the ground up.
This gives the developer(s) the ability to use this API within an async/await context, but not forcing them to.
It makes it so much easier than having to deal with callbacks in your code.
You can view the [example](https://github.com/bamilab/browser-window/tree/master/example) to see how easy a graphical user interface within a asynchronous context can be made.

## Requirements

Browser Window currently relies on [CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home) as its browser engine.
Also, only windows is the only supported platform at the moment.

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

If you want to help out, you're more than welcome! I will need someone at some point to implement iOS support.

## Comming Soon

This project is very new. At the moment only very basic functionality is made available. There will be more soon:

* Linux support
* Cookie support (including HttpOnly cookies)
