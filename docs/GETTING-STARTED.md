# Getting Started

_Browser Window_ currently relies on [CEF3](https://bitbucket.org/chromiumembedded/cef/wiki/Home).
You will also need [cmake](https://cmake.org/).
And on Windows, you will also need _Visual Studio_.
You can download the free _Community_ version [here](https://visualstudio.microsoft.com/vs/).

If you want to set up CEF by building it from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/MasterBuildQuickStart.md).
However, it will take a lot of effort and time, memory & disk space for the compilation process.

## Using the binary distribution

### Download & Extract

The easiest and quickest way to set up CEF is to get the binary distribution.
You can get the latest prebuilt binaries [here](https://cef-builds.spotifycdn.com/index.html).
The minimal version will be fine.
Version `121.3.13` is the lastest version known to work.
Once downloaded, you will need to extract it somewhere.

### Compilation

The library itself is already compiled for the binary distribution. However, there is a static 'wrapper' library that still needs to be built.
To do this, first run _cmake_ by running this on the command line from within the extracted folder:
```
cmake .
```

### Unix-like Operating Systems

After you have run `cmake`, you can just simply run `make`.
This will build CEF.

### Windows

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
Otherwise, `cargo` will fail to build `browser-window`.

You also need to copy resource and library files (`.so`/`.dll` and `.bin`), so that they are available to the executable.
Running `setup-cef-files.sh` or `setup-cef-files.bat` does all this for you.
It will also change some permissions that CEF requires you to do.
`setup-cef-files.sh` will use `sudo`, so if you don't have `sudo`, inspect the file and execute the commands yourself.

For the library files, you could also just add the `Release` folder to your `PATH` environment variable for the `.so`/`.dll` files.

That's it!
Running `cargo run` should now work.

If you encounter any issues, take a look at the [issue diagnosis page](https://github.com/bamilab/browser-window/blob/master/docs/ISSUE-DIAGNOSIS.md).
If that doesn't help, you can submit an issue to the repository.

## Building CEF From Source

If you really want to build CEF from source, take a look at [this](https://bitbucket.org/chromiumembedded/cef/wiki/BranchesAndBuilding.md#markdown-header-automated-method).
You still need to set up the environment variables and copy the files into the working directory afterwards.
