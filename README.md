# Browser Window

Browser Window is a simple Rust crate for working with windows that are actually browsers.
Just like [Electron](https://www.electronjs.org/), you can build graphical user interfaces with HTML/CSS/JS technology, but you can also use it to just have some browser functionality in your application.

Browser Window was born from the lack of a good and simple Electron alternative for Rust.
There are actually a few ones out there already.
However, they lack a few important things.

## The problems Browser Window is solving

The main problem is that they (except on Linux) use browser engine's already available on the operating system.
This is useful if you want to ship your application as a single executable file.
However, using different engines on each platform makes it harder to make your application work consistently on each platform, especially when using newer features that are not yet supported by some browsers engines.
Using the same engine for each platform is a good idea.

Another problem you run into when using preinstalled browser engines, is that in the case of Windows, that engine is not being maintained anymore.
Windows uses the EdgeHTML engine that Edge used before it switched to using CEF as well.
If you want to use newest HTML5/CSS3 features, you shouldn't rely on an old browser engine.

Moreover, some other simple features that would make life so much better, are unfortunately not available in other crates.
For example, getting the resulting output back from a JavaScript evaluation is not self-evident.

Another thing is, as multi-core processors are becoming the norm, multi-threading is becoming more and more relevant.
GUI libraries are often single-threaded by design, and need some way of delegating heavy work to another thread.
Fortunately, Rust has a pretty mechanism for writing asynchronous code with its async/await syntax, which can be extremely efficient as well.
We'd be wrong not to utilise this!

So Browser Window is designed with asynchronous programming in mind from the ground up.
The developer(s) have the ability to use this API within an async/await environment without much hassle.
Single-threaded applications are also still possible.
You can view the [example](https://github.com/bamilab/browser-window/tree/master/example) to see how easily a GUI with Browser Window can be made, within a multi-threaded environment.

## Requirements

At this moment, Windows is the only supported platform, but support for other platforms will follow.

Also, there needs to be one browser engine embedding library available.
There currently are two options:
* [CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home) (the Chromium Embedding Framework)
* [Edge WebView](https://docs.microsoft.com/en-us/microsoft-edge/hosting/webview) (only for Windows 10)

### Comparison

CEF is used as the default engine for Browser Window.
It is being actively maintained and supports multiple platforms.
So when Browser Window has multiple platform support in the future, CEF is available on all of them.

Edge WebView is not being maintained anymore, in light of the new Edge WebView2 engine having just been released. (Which is just a wrapper for CEF actually.)
However, it is already available on your Windows 10 system, so no additional setup is needed.
This makes it possible to ship your executable as a single .exe file because of this.

Support for cookies will be implemented in the future.
However, Edge's WebView has this problem where no HTTP Only cookies are available, which kind of defeats the purpose of this feature.

## Getting Started

Click [here](./docs/getting-started/WINDOWS.md) for a manual on how to set up everything that's needed to use Browser Window.

## License

This software is available as open source software under a MIT license, for maximum freedom.

## Development

If you want to help out, you're more than welcome! I could use some help with implementing MacOS support actually.

## Comming Soon

At the moment, basic functionality is available, but there is more to come.
These are the features that are awaiting implementation:

* Linux support (through GTK+)
* MacOS support (through Cocoa)
* Cookie support (including HttpOnly cookies)
