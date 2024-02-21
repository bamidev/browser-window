# Browser Window

_Browser Window_ is a simple Rust crate for utilizing a browser engine to create a graphical user interface.
Just like [Electron.js](https://www.electronjs.org/), you can build your GUI with HTML, CSS & JS, or simply have some basic browser functionality at your disposal.
_Browser Window_ was born from the lack of a good and simple Electron alternative for Rust.

## Design Principles

_Browser Window_ is designed to be simple to use, and to work cross-platform.
It is built to utilize Rust's async/await syntax fully, so that exchanging data between the browser and your application can be straightforward, instead of using messy callbacks.

With other alternative crates, you can't just simply invoke a JavaScript function and return its output in one line like this:
```
let js_return_value = my_window.eval_js("my_js_func()").await.unwrap();
```
Something similar is available in _Electron_ as well, and this is just something that is paramount to making two-way communication between the host-code (Rust) and client-code (JavaScript) comfortable.

Moreover, if you wish to use it in a multi-threaded environment, you can do that as well.
There are thread-safe handles available for easy exchange of data & work between the GUI thread and others.

You can view some [example](https://github.com/bamilab/browser-window/tree/master/examples) to get some ideas of what is possible with _Browser Window_.
There is an example of a basic terminal emulator, and an example of an authentication dialog that fetches the session cookie.

## Requirements

_Browser Window_ relies on the [Chromium Embedding Framework](https://bitbucket.org/chromiumembedded/cef/wiki/Home), or CEF in short.
_Browser Window_ works on any platform that is also supported by CEF.
Currently those are Linux, macOS and Windows, but macOS needs some testing.

However, that does mean it requires a bit of effort to set things up.
Also, due to CEF's frequent release schedule, be sure to pick a version that is _compatible_ with Browser Window, or try a new version at your own risk.

The latest CEF version that is compatible is: 121.3.13

## Getting Started

Click [here](./docs/GETTING-STARTED.md) for a manual on how to set up everything to be able to compile and run you app with _Browser Window_.

## License

This software is available as open source software under a MIT license, for maximum freedom and minimum restrictions.

## Development

If you want to help out, you're more than welcome!

## Future Plans

At the moment, there is a decent set of functionality available, but if something is lacking, please [submit an issue](https://github.com/bamilab/browser-window/issues), and I'll take a look at it.

Otherwise, there are some things that are yet to come:

* Events [underway]
* Servo engine support (currently missing good embedding support)
