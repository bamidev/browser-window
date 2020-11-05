# Getting Started on Windows

##### The TL;DR Solution

If you want to get started as quick as possible, all you have to do is enable feature "edge" in your crate's Cargo.toml. Although, this does require Windows 10.

```
[dependencies]
browser-window = { features = ["edge"] }
```

This compiles Browser Window with the EdgeHTML engine on Windows.
However, it is recommended to compile it with CEF3 (Chromium Embedding Framework), for better support of newer HTML5 features. No crate features need to be enabled when using CEF.

##### Download & Extract

The easiest and quickest way to set up CEF is to get the binary distribution.
Building the source code is very time consuming.
You can get the latest prebuilt binaries [here](http://opensource.spotify.com/cefbuilds/index.html#windows64).
The minimal version is fine.
You can also get 32-bit binaries if you really want.

You need to extract this archive.
These .tar.bz files are not supported by Windows itself.
You could use the good old [WinRAR](https://www.rarlab.com/download.htm) to extract this.

##### Environment Variables & Resource Files

Once you have extracted everything, we need to set up some environment variables, so that the MSVC compiler knows where it can find the header and library files.
You can find your system environment variables by going to: Control Panel > System and Security > System > Advanced system settings.
Then click on the "Environment variables..." button.

* Add the extracted folder to your %INCLUDE% variable.
* Add the Release folder inside that extracted folder to your %LIB% variable.
* Do one of the following:
    1. Add the same Release folder to your %PATH% variable.
    2. Copy all .dll files in the Release folder to the executables working directory.
       This is target/debug or target/release within your crate's folder.
* Copy all .bin files from the Release folder, into target/debug or target/release.
* Copy all files from the Resource folder, into the same folder.

That's it!

##### Building From Source

If you really want to build CEF from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md#markdown-header-windows-setup).
You still need to set up the environment variables and copy the files into the working directory afterwards.
