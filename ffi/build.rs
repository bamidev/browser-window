extern crate cc;

use std::env;



fn main() {
    let mut build = cc::Build::new();

    let target = env::var("TARGET").unwrap();

	let mut std_flag = "-std=c11";
	
	if target.contains("windows") {
		build
			.file("src/win32.c")
			.file("src/application/win32.c")
			.file("src/browser_window/common_with_window.c")
			.file("src/window/common.c")
			.file("src/window/win32.c")
			.flag("/D")
			.flag("BW_WIN32");
	}

	// Note: not actually supported atm
	if cfg!(feature = "webview2") {
		if target.contains("windows") {
			std_flag = "/std:c++17";

			build
				.file("src/browser_window/webview2.cpp")
				.flag("/D")
				.flag("BW_WEBVIEW2");
		}
	}
	else {	// Use CEF if no other engine is selected
		std_flag = "/std:c++17";

		build
			.file("src/application/cef.cpp")
			.file("src/browser_window/cef.cpp")
			.file("src/cef/bw_handle_map.cpp")
			.file("src/cef/exception.cpp")
			.flag("/D")
			.flag("BW_CEF");

		if target.contains("windows") {
			build
				.flag("/D")
				.flag("BW_CEF_WINDOWS");
				//.flag("/MT");
		}
	}

	build
		.file("src/string.c")
		.file("src/browser_window/common.c")
		.file("src/window/common.c")
		.file("src/err.c")
		.flag_if_supported( std_flag )
		.compile("browser_window");
}
