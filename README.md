# Browser Window

Browser Window is a simple Rust crate for using Browsers in simple windows.
Just like [Electron](https://www.electronjs.org/), you can use it to build graphical user interfaces with HTML/CSS/JS technology.
Browser Window was born from the lack of a good and simple Electron alternative for Rust.
There are actually a few ones out there already.
However, they lack a few important things.

## Goals

Browser Window aims to be cross-platform, very simple, and straight forward.
Many methods in Browser Window are asynchronous, such as evaluating JavaScript code and getting back its output.
Browser Window utilizes and benefits from Rust's async/await syntax, so this should make it a breeze.

Moreover, if you want to use it in a multi-threaded environment, that's also possible with our thread-safe handles!

You can view an [example](https://github.com/bamilab/browser-window/tree/master/example) of a terminal emulator, to see how easily a GUI with Browser Window can be made.

## Requirements

At this moment, Windows is the only supported platform, but support for other platforms will follow.

Also, Browser Window uses the Chromium engine. So the [Chromium Embedding Framework](https://bitbucket.org/chromiumembedded/cef/wiki/Home) is a required dependency.

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
