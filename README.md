# BrowserWindow

_BrowserWindow_ is a simple Rust crate for utilizing a browser engine to create a graphical user interface.
Just like [Electron.js](https://www.electronjs.org/), you can build your GUI with HTML, CSS & JS, or simply have some basic browser functionality at your disposal.
_BrowserWindow_ was born from the lack of a good and simple Electron alternative for Rust.

## Introduction

_BrowserWindow_ is designed to be easy to use, and work cross-platform. It is built to work in Rust
applications that utilize async/await syntax - or that don't that at all.
One of the following underlying browser embedding frameworks can be selected: CEF or WebkitGTK.

CEF (or the Chromium Embedding Framework) is recommended when used on Windows.
WebkitGTK is recommended when used on Unix-like systems, or when cross-compiling.
Enable feature `cef` or `webkitgtk` to select either one.

If you're wondering why _BrowserWindow_ doesn't actually support the pre-installed EdgeHTML
framework for Windows: EdgeHTML is deprecated in favor of the Microsoft Edge WebView2, but the
latter needs to be uninstalled manually be the end-user, defeating it's utility over CEF, if you're planning to ship your application to Windows users anyway.

Moreover, if you wish to use it in a multi-threaded environment, you can do that as well.
There are thread-safe handles available for easy exchange of data & work between the GUI thread and others.

You can view some [example](https://github.com/bamilab/browser-window/tree/master/examples) to get
some ideas of what is possible with _BrowserWindow_.

## Requirements

The underlying framework, be it CEF or WebkitGTK, needs to be installed on your system. There is a
[guide](./docs/GETTING-STARTED.md) to get your started.

Also keep in mind that latest known CEF version that is compatible with this version of
_BrowserWindow_ is: v121.3.13

## Getting Started

Click [here](./docs/GETTING-STARTED.md) for a manual on how to set up everything to be able to
compile and run you app with _BrowserWindow_.

## License

This software is available as open source software under a MIT license, for maximum freedom and
minimum restrictions.

## Future Plans

At the moment, there is a decent set of functionality available, but if something is lacking, please [submit an issue](https://github.com/bamilab/browser-window/issues), and I'll take a look at it.
There are not a lot of functions exposed in the BrowserWindow object. If you need a feature that
isn't there, let me know!

Otherwise, there are some things that are yet to come:

* Events
* Support for Webkit with Cocoa on MacOS.
