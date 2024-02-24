# BrowserWindow

_BrowserWindow_ is a simple Rust crate for utilizing a browser engine to create a graphical user interface.
Just like [Electron.js](https://www.electronjs.org/), you can build your GUI with HTML, CSS & JS, or simply have some basic browser functionality at your disposal.
_BrowserWindow_ was born from the lack of a good and simple Electron alternative for Rust.

## Introduction

_BrowserWindow_ is designed to be easy to use, and work cross-platform. It is built to work in Rust
applications that utilize async/await syntax - or that don't do that at all. It even has optional
thread-safe handles. There are currently two different underlying browser frameworks that can be
selected: WebkitGTK or CEF.

CEF (or the Chromium Embedding Framework) is recommended when used on Windows, but requires more
effort to set up.
WebkitGTK is recommended when used on Unix-like systems or when cross-compiling, and is genereally
pretty easy to set up.
Enable feature `cef` or `webkitgtk` to select either one.

Moreover, if you wish to use it in a multi-threaded environment, you can do that as well.
There are thread-safe handles available for easy exchange of data & work between the GUI thread and others.

You can look at some [examples](https://github.com/bamilab/browser-window/tree/master/examples) to
get an idea how you can use the api.

## Getting Started

The underlying framework, be it CEF or WebkitGTK, needs to be installed on your system. There is a
[guide](./docs/GETTING-STARTED.md) to get you started with using _BrowserWindow_ on your own system.

## License

This software is available as open source software under a MIT license, for maximum freedom and
minimum restrictions.

## Future Plans

At the moment, there is a decent set of functionality available, but if something is lacking, please [submit an issue](https://github.com/bamilab/browser-window/issues), and I'll take a look at it.
There are not a lot of functions exposed in the BrowserWindow object. If you need a feature that
isn't there, let me know!

Otherwise, there are some things that are yet to come:

* Support for the Microsoft Edge WebView2 framework.
* Events
* Support for Webkit with Cocoa on MacOS.
