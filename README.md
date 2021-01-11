# Browser Window

_Browser Window_ is a simple Rust crate for utilizing a browser engine to create a graphical user interface.
Just like [Electron](https://www.electronjs.org/), you can build your GUI with HTML/CSS/JS, or simply have some browser functionality.
_Browser Window_ was born from the lack of a good and simple Electron alternative for Rust.

## Goals

Browser Window aims to be cross-platform, very simple, and straight forward.
Some methods in Browser Window are asynchronous, such as evaluating JavaScript code and getting back its output.
Browser Window utilizes and benefits from Rust's async/await syntax, so this should make it a breeze.

Moreover, if you want to use it in a multi-threaded environment, you can.
There are thread-safe handles available for easy exchange of data and work between the GUI thread and others.

You can view an [example](https://github.com/bamilab/browser-window/tree/master/example) of a terminal emulator, to see how easily a GUI with Browser Window can be made.

## Requirements

At this moment, Windows is the only supported platform, but support for other platforms will follow.

Also, _Browser Window_ relies on the [Chromium Embedding Framework](https://bitbucket.org/chromiumembedded/cef/wiki/Home).

## Getting Started

Click [here](./docs/getting-started) for a manual on how to set up everything that's needed to use Browser Window.

## License

This software is available as open source software under a MIT license, for maximum freedom.

## Development

If you want to help out, you're more than welcome! I could use some help with implementing MacOS support actually.

## Comming Soon

At the moment, only basic functionality is available, but there is more to come.
These are the features that are awaiting implementation:

* Linux support (through GTK+) [underway]
* MacOS support (through Cocoa)
* Cookie support (including HttpOnly cookies)
* Events [underway]
* Servo engine support
