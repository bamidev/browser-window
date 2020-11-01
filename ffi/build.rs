extern crate cc;

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

	if target.contains("windows") {
		std_flag = "/std:c++17";

		// Win32 source files
		build
			.file("src/win32.c")
			.file("src/application/win32.c")
			.file("src/window/common.c")
			.file("src/window/win32.c")
			.define("BW_WIN32", None);

		// CEF source files
		build
			.file("src/application/cef.cpp")
			.file("src/browser_window/cef.cpp")
			.file("src/cef/bw_handle_map.cpp")
			.file("src/cef/exception.cpp")
			.define("BW_CEF", None);
	}

	// Common source files
	build
		.file("src/string.c")
		.file("src/browser_window/common.c")
		.file("src/window/common.c")
		.file("src/err.c")
		.flag_if_supported( std_flag )
		.compile("browser_window");
}
