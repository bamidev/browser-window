extern crate cc;
extern crate pkg_config;

use std::env;



fn main() {

	// If this is being build by docs.rs, don't do anything.
	// docs.rs is not able to compile the C/C++ source files because it doesn't have the win32 and cef3 header files available.
	if let Ok(_) = env::var("DOCS_RS") {
		return
	}

    let mut build = cc::Build::new();

    let target = env::var("TARGET").unwrap();

	let mut std_flag = "-std=c11";

	/**************************************
	 *	The Platform source files
	 **************************************/
	if target.contains("windows") {
		// Win32 API
		std_flag = "/std:c++17";

		build
			.file("src/win32.c")
			.file("src/application/win32.c")
			.file("src/window/common.c")
			.file("src/window/win32.c")
			.define("BW_WIN32", None);
	}
	else {
		pkg_config::Config::new().atleast_version("3.0").probe("gtk+-3.0").unwrap();

		build
			//.file("src/application/gtk.c")
			.define("BW_GTK", None);
	}

	/**************************************
	 *	The Browser Engine source files
	 **************************************/
	if cfg!(feature = "edge") {
		// Egde WebView
		if !target.contains("windows") {
			panic!("The Edge WebView api only works on Windows!");
		}

		build
			.file("src/application/edge.cpp")
			.file("src/browser_window/edge.cpp")
			.define("BW_EDGE", None);
	}
	else {
		// CEF3
		build
			.file("src/application/cef.cpp")
			.file("src/browser_window/cef.cpp")
			.file("src/cef/bw_handle_map.cpp")
			.file("src/cef/exception.cpp")
			.define("BW_CEF", None);
	}

	/**************************************
	 *	All other source files
	 **************************************/
	build
		.file("src/string.c")
		.file("src/browser_window/common.c")
		.file("src/window/common.c")
		.file("src/err.c")
		.flag_if_supported( std_flag )
		.compile("browser_window");
}
