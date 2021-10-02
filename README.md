# Browser Window

_Browser Window_ is a simple Rust crate for utilizing a browser engine to create a graphical user interface.
Just like [Electron.js](https://www.electronjs.org/), you can build your GUI with HTML, CSS & JS, or simply have some browser functionality at your disposal.

_Browser Window_ was born from the lack of a good and simple Electron alternative for Rust.
There are other crates out there doing a similar thing, but they lack a few important key features.

For example, other alternatives tend to depend on the target platform's native browser engine, initially intended to have something that works _out-of-the-box_.
However, this poses a few problems.
For one, Linux distributions don't have a _native_ browser engine, so Linux users still need to install libraries.
Moreover, the browser engine that is shipped with Windows nowadays is now old and deprecated.
So therefor, the end-user is still required to install an extra component to get your application to work on their system.
So only MacOS just works supposedly.

But you still have the issue of having to deal with the fact that not every browser engine behaves the same.
This is the same issue web developers face, although it isn't as bad anymore as it used to.
Nevertheless, when using one browser engine for all platforms, you don't have to worry about all that anyway.
If you don't use platform dependent JavaScript or Rust, and it works, it works everywhere.
End of story.

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
Currently those are Linux, macOS and Windows, but macOS has not been tested so far.

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
