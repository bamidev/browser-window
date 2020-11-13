# Browser Window

Browser Window is a simple Rust crate for working with windows that are actually browsers.
Just like [Electron](https://www.electronjs.org/), you can build graphical user interfaces with HTML/CSS/JS technology, but you can also use it to just have some browser functionality in your application.

Browser Window was born from the lack of a good and simple Electron alternative for Rust.
There are actually a few ones out there already.
However, they lack a few important things.

## Goals

Browser Window aims to be cross-platform, very simple, and straight forward.
Many methods in Browser Window are asynchronous, such as evaluating JavaScript code and getting back its output.
Browser Window utilizes and benefits from Rust's async/await syntax, so this should make it a breeze.

Moreover, multi-threading is becoming more and more important.
If you need to access the GUI from other threads, Browser Window also provides thread-safe handles to do this.

You can view the [example](https://github.com/bamilab/browser-window/tree/master/example) to see how easily a GUI with Browser Window can be made, within a single-threaded environment.

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

Click [here](./docs/getting-started) for a manual on how to set up everything that's needed to use Browser Window.

## License

This software is available as open source software under a MIT license, for maximum freedom.

## Development

If you want to help out, you're more than welcome! I could use some help with implementing MacOS support actually.

## Comming Soon

At the moment, basic functionality is available, but there is more to come.
These are the features that are awaiting implementation:

* Linux support (through GTK+) [underway]
* MacOS support (through Cocoa)
* Cookie support (including HttpOnly cookies)
* Events
