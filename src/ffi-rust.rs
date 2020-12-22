//! This module contains definitions for C functions that are missing in browser-window-ffi with certain configurations.
//! For example, when using webrenderer as the browser engine, the implementations of `bw_BrowserWindowImpl_*` functions are missing, and implemented within Rust.

mod servo