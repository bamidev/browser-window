# Getting Started on Windows

_Browser Window_ currently relies on [CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home).
You will also need [cmake](https://cmake.org/), which you can download [here](https://cmake.org/download/).
And you will need _Visual Studio_. You can download the free _Community_ version [here](https://visualstudio.microsoft.com/vs/).

If you want to set up CEF by building it from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md).
However, it will take a lot of time, memory and disk space.

## Using the binary distribution

### Download & Extract

The easiest and quickest way to set up CEF is to get the binary distribution.
Building the source code is very, very time and resource consuming.
You can get the latest prebuilt binaries [here](http://opensource.spotify.com/cefbuilds/index.html#windows64).
The minimal version will be fine.
You can also get the 32-bit binaries if you really want.

Once downloaded, you will need to extract it somewhere
These .tar.bz files are not readable by Windows itself.
Good-old [WinRAR](https://www.rarlab.com/download.htm) should do the trick.

### Compilation

The library itself is already compiled for the binary distribution. However, there is a static 'wrapper' library that still needs to be built.
To do this, first run _cmake_ by running this on the command line from within the extracted folder:
```
cmake .
```

A newly generated Visual Studio solution has been generated in the folder.
You should build this solution's Release target.
However, before you do, you need to change one setting in the project's settings.

Goto Project -> Properties -> Configuration Properties -> C/C++ -> Code Generation

If `Runtime Library` is set to `Multi-threaded (/MT)`, set it to `Multi-threaded DLL (/MD)`.
Rust links against the C runtime dynamically, and thus requires CEF to link to it dynamically as well.

Now you can build the solution.

## Environment Variables & Resource Files

Once you have extracted and compiled everything, we need to let _Browser Window_ know where it can find the header and library files to link to.
If you set environment variable `CEF_PATH` to the directory that you have extracted, Browser Window is able to find them.

You also need to copy all resource files, found in the Resource directory, to the executables working directory.

To last thing that needs to be done is that the library files need to be made available to the executable.
Copy all .dll and .bin files to the working directory as well.
You could also just add the Release folder to your `%PATH%` environment variable for the .dll files.

That's it!
A call to `cargo run` will do it.

If you encounter any issues, take a look at the [issue diagnosis page](https://github.com/bamilab/browser-window/blob/master/docs/ISSUE-DIAGNOSIS.md).

## Building From Source

If you really want to build CEF from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/BranchesAndBuilding.md#markdown-header-automated-method).
You still need to set up the environment variables and copy the files into the working directory afterwards.
