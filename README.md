# Browser Window

_Browser Window_ is a simple Rust crate for utilizing a browser engine to create a graphical user interface.
Just like [Electron](https://www.electronjs.org/), you can build your GUI with HTML/CSS/JS, or simply have some browser functionality.
_Browser Window_ was born from the lack of a good and simple Electron alternative for Rust.

## Goals

Browser Window aims to be cross-platform, very simple, and straight forward.
Some methods in Browser Window are asynchronous, such as evaluating JavaScript code.
Browser Window is built to utilize Rust's async/await syntax, to keep your code and logic as simple as possible.
This also makes it possible to communicate smoothly with JavaScript on the client side.

Moreover, if you want to use it in a multi-threaded environment, you can.
There are thread-safe handles available for easy exchange of data and work between the GUI thread and others.

You can view an [example](https://github.com/bamilab/browser-window/tree/master/example) of a terminal emulator, to see how easily a GUI made with Browser Window can be done.

## Requirements

_Browser Window_ relies on the [Chromium Embedding Framework](https://bitbucket.org/chromiumembedded/cef/wiki/Home), or CEF.
Browser Window works on any platform that is also supported by CEF.
Currently that is Linux, macOS and Windows, but macOS has not been tested yet.

However, due to issues that CEF has with its GTK support, the Linux and macOS implementations use CEF's own windowing API, which is unfortunately is pretty limited.
So window manipulation itself is currently only mostly supported on Windows.

## Getting Started

Click [here](./docs/GETTING-STARTED.md) for a manual on how to set up everything to be able to use Browser Window.

## License

This software is available as open source software under a MIT license, for maximum freedom and minimum restrictions.

## Development

If you want to help out, you're more than welcome!

## Future Plans

At the moment, only basic functionality is available, but there is more to come.
These are features that are planned to be implemented at some point:

* Cookie support (including HttpOnly cookies)
* Events [underway]
* Servo engine support (currently missing good embedding support)
