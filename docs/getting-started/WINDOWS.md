# Getting Started on Windows

##### Download & Extract

The easiest and quickest way to set up CEF is to get the binary distribution.
Building the source code is very, very time consuming.
You can get the latest prebuilt binaries [here](http://opensource.spotify.com/cefbuilds/index.html#windows64).
The minimal version will be fine.
You can also get the 32-bit binaries if you really want.

You need to extract this archive.
These .tar.bz files are not supported by Windows itself.
Perhaps [WinRAR](https://www.rarlab.com/download.htm) will do.

##### Environment Variables & Resource Files

Once you have extracted everything, we need to let Browser Window know where it can find the header and library files to link to.
If you set environment variable `CEF_PATH` to the directory that you have extracted, Browser Window is able to find them.

You also need to copy all resource files, found in the Resource directory, to the executables working directory.

To last thing that needs to be done is that the library files need to be made available to the executable.
Copy all .dll and .bin files to the working directory as well.
You could also just add the Release folder to your `%PATH%` environment variable for the .dll files.

That's it!
A call to `cargo run` will do it.

##### Building From Source

If you really want to build CEF from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/BranchesAndBuilding.md#markdown-header-automated-method).
You still need to set up the environment variables and copy the files into the working directory afterwards.
