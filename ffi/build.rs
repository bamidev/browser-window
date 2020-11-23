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

	let std_flag =
		if target.contains("windows") {
			"/std:c++17"
		}
		else {
			"-std=c++17"
		};

	/**************************************
	 *	The Platform source files
	 **************************************/
	if target.contains("windows") {

		// Win32 API
		build
			.file("src/win32.c")
			.file("src/application/win32.c")
			.file("src/window/win32.c")
			.define("BW_WIN32", None)
			.define("_CRT_SECURE_NO_WARNINGS", None);	// Disable sprintf_s warnings. sprintf_s tends to cause segfaults.
	}
	// Non-Windows platforms use GTK:
	else {
		match pkg_config::Config::new().atleast_version("3.0").arg("--cflags").probe("gtk+-3.0") {
			Err(e) => panic!("Unable to find GTK 3 development files: {}", e),
			Ok( lib ) => {

				// Manually add GTK includes to compiler
				for inc in &lib.include_paths {
					build.include( inc );
				}

				build
					.file("src/application/gtk.c")
					.file("src/window/gtk.c")
					.define("BW_GTK", None);
			}
		}
	}

	/**************************************
	 *	The Browser Engine (CEF3) source files
	 **************************************/
	// Make sure CEF_PATH is set
	match env::var("CEF_PATH") {
		Err(e) => {
			match e {
				env::VarError::NotPresent => panic!("Environment variable CEF_PATH is not set! This is needed by Browser Window to find CEF's development files. See https://github.com/bamilab/browser-window/tree/master/docs/getting-started for more information."),
				other => panic!("Unable to use CEF_PATH: {}", other)
			}
		},
		Ok(cef_path) => {
			build.include(&cef_path);
			println!("cargo:rustc-link-search={}/libcef_dll_wrapper", &cef_path);
			println!("cargo:rustc-link-search={}/Release", &cef_path);
			if target.contains("msvc") {
				println!("cargo:rustc-link-lib=static={}", "libcef_dll_wrapper");
				println!("cargo:rustc-link-lib=dylib={}", "libcef");
			} else {
				println!("cargo:rustc-link-lib=static={}", "cef_dll_wrapper");
				println!("cargo:rustc-link-lib=dylib={}", "cef");
			}
		}
	}

	// CEF source files
	build
		.file("src/application/cef.cpp")
		.file("src/browser_window/cef.cpp")
		.file("src/cef/bw_handle_map.cpp")
		.file("src/cef/client_handler.cpp")
		.file("src/cef/exception.cpp")
		.define("BW_CEF", None)
		.cpp(true);

	/**************************************
	 *	All other source files
	 **************************************/
	build
		.file("src/application/common.c")
		.file("src/browser_window/common.c")
		.file("src/err.c")
		.file("src/string.c")
		.file("src/window/common.c")
		.flag( std_flag )
		.compile("browser_window");
}
