extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::fs;
use std::path::PathBuf;



fn rerun_if_directory_changed<P>( _path: P ) where P: Into<PathBuf> {
	let path: PathBuf = _path.into();

	let dir_iterator = match fs::read_dir( &path ) {
		Err(e) => panic!( format!("Unable to read directory: {}", e) ),
		Ok( iterator ) => iterator
	};

	for sub_path in dir_iterator {

		match sub_path {
			Err(e) => panic!( format!("Unable to read directory entry for dir {}: {}", path.as_os_str().to_str().unwrap(), e) ),
			Ok( entry ) => {

				if entry.path().is_dir() {
					rerun_if_directory_changed( entry.path() );
				}
				else {
					println!("cargo:rerun-if-changed={}", entry.path().as_os_str().to_str().unwrap() );
				}
			}
		}
	}
}

fn main() {

	// If this is being build by docs.rs, don't do anything.
	// docs.rs is not able to compile the C/C++ source files because it doesn't have the win32 and cef3 header files available in their docker system in which they test-build.
	if let Ok(_) = env::var("DOCS_RS") {
		return
	}

	println!("cargo:rerun-if-env-changed=CEF_PATH");
	rerun_if_directory_changed("src");

	let out_path = PathBuf::from( env::var("OUT_DIR").expect("Unable to get output directory for FFI bindings") );
	let target = env::var("TARGET").unwrap();

	let mut build = cc::Build::new();
	let std_flag =
		if target.contains("windows") {
			"/std:c++17"
		}
		else {
			"-std=c++17"
		};

	/**************************************
	 *	C header files for bindgen
	 **************************************/
	let mut bindgen_builder = bindgen::Builder::default()
		.clang_arg("-DBW_CEF")
		.clang_arg("-DBW_BINDGEN")
		.header("src/application.h")
		.header("src/browser_window.h")
		.header("src/common.h")
		.header("src/err.h")
		.header("src/string.h")
		.header("src/window.h");
		//.parse_callbacks(Box::new(bindgen::CargoCallbacks));

	/**************************************
	 *	The Platform source files
	 **************************************/
	if target.contains("windows") {

		bindgen_builder = bindgen_builder.clang_arg("-DBW_WIN32");

		// Win32 API
		build
			.file("src/win32.c")
			.file("src/application/win32.c")
			.file("src/window/win32.c")
			.define("BW_WIN32", None)
			.define("_CRT_SECURE_NO_WARNINGS", None);	// Disable sprintf_s warnings. sprintf_s tends to cause segfaults.
	}
	// Non-Windows platforms:
	else {

		bindgen_builder = bindgen_builder.clang_arg("-DBW_GTK");

		// GTK source files
		build
			.file("src/application/gtk.c")
			.file("src/window/gtk.c")
			.define("BW_GTK", None);

		match pkg_config::Config::new().atleast_version("3.0").arg("--cflags").probe("gtk+-3.0") {
			Err(e) => panic!("Unable to find GTK 3 development files: {}", e),
			Ok( lib ) => {

				// Manually add GTK includes to compiler and bindgen
				for inc in &lib.include_paths {
					build.include( inc );
					bindgen_builder = bindgen_builder.clang_arg( format!("-I{}", inc.as_os_str().to_str().unwrap()) );
				}
			}
		}
	}

	/**************************************
	 *	The Browser Engine (CEF3) source files
	 **************************************/
	build.flag_if_supported("-Wno-unused-parameter");	// CEF's header files produce a lot of unused parameters warnings.
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

	// Let bindgen generate the bindings
	bindgen_builder
		.generate().expect("Unable to generate FFI bindings!")
		.write_to_file( out_path.join("c_bindings.rs") ).expect("Unable to write FFI bindings to file!");

	/**************************************
	 *	CEF source files
	 **************************************/
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
