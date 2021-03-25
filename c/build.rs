extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::fs;
use std::path::PathBuf;



#[derive(Debug)]
struct BwBindgenCallbacks {}



/// Prints all compiler commands to rerun if any file has changed within the given directory or a subdictory thereof.
fn rerun_if_directory_changed<P>( _path: P ) where P: Into<PathBuf> {
	let path: PathBuf = _path.into();

	let dir_iterator = match fs::read_dir( &path ) {
		Err(e) => panic!( "Unable to read directory: {}", e ),
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

	println!("cargo:rerun-if-env-changed=CEF_PATH");
	rerun_if_directory_changed("src");

	let target = env::var("TARGET").unwrap();
	let out_path = PathBuf::from( env::var("OUT_DIR").expect("Unable to get output directory for C/C++ code base crate") );
	let backup_file = PathBuf::from( env::var("CARGO_MANIFEST_DIR").unwrap() + "/bindgen_backup.rs" );

	// Workaround for docs.rs
	// Docs.rs is not able to compile the C/C++ source files because it doesn't have the win32 and cef header files available in their docker system in which they test-build.
	if let Ok(_) = env::var("DOCS_RS") {
		fs::copy( &backup_file, out_path.join("c_bindings.rs") ).expect("Unable to copy backup c bindings");
		return
	}

	let mut build = cc::Build::new();
	let std_flag = if cfg!(feature = "cef") {
		if target.contains("msvc") {
			"/std:c++17"
		}
		else {
			"-std=c++17"
		}
	}
	else {
		if target.contains("msvc") {
			"/std:c11"
		}
		else {
			"-std=c11"
		}
	};

	/**************************************
	 *	C header files for bindgen
	 **************************************/
	 let mut bgbuilder = bindgen::Builder::default()
	 	.parse_callbacks( Box::new( BwBindgenCallbacks {} ) )
		.clang_arg("-DBW_BINDGEN")
		.header("src/application.h")
		.header("src/browser_window.h")
		.header("src/common.h")
		.header("src/err.h")
		.header("src/string.h")
		.header("src/window.h");

	/**************************************
	 *	The Platform source files
	 **************************************/
	if target.contains("windows") {
		bgbuilder = bgbuilder.clang_arg("-DBW_WIN32");

		// Win32 API
		build
			.file("src/win32.c")
			.file("src/application/win32.c")
			.file("src/window/win32.c")
			.define("BW_WIN32", None)
			.define("_CRT_SECURE_NO_WARNINGS", None);	// Disable sprintf_s warnings. sprintf_s tends to cause segfaults...
	}
	// When opting for using GTK:
	else if cfg!(feature = "gtk") {
		bgbuilder = bgbuilder.clang_arg("-DBW_GTK");

		// GTK source files
		build
			.file("src/application/gtk.c")
			.file("src/window/gtk.c")
			.define("BW_GTK", None);

		match pkg_config::Config::new().atleast_version("3.0").arg("--cflags").probe("gtk+-3.0") {
			Err(e) => panic!("Unable to find GTK 3 development files: {}", e),
			Ok( lib ) => {

				// Manually add GTK includes to compiler
				for inc in &lib.include_paths {
					build.include( inc );
				}
			}
		}
	}
	// Non-windows systems that opt for using CEF, use CEF's own internal windowing features
	else if cfg!(feature = "cef") {
		bgbuilder = bgbuilder.clang_arg("-DBW_CEF_WINDOW");
		build
			.file("src/application/cef_window.cpp")
			.file("src/window/cef.cpp")
			.define("BW_CEF_WINDOW", None);
	}

	/**************************************
	 *	The Browser Engine (CEF3) source files
	 **************************************/
	if cfg!(feature = "cef") {
		bgbuilder = bgbuilder.clang_arg("-DBW_CEF");

		build.flag_if_supported("-Wno-unused-parameter");	// CEF's header files produce a lot of unused parameters warnings.
		match env::var("CEF_PATH") {
			Err(e) => {
				match e {
					env::VarError::NotPresent => {

						// Disable checking CEF_PATH for the docs.rs compiler, it is not on their system anyway.
						if let Err(_) = env::var("DOCS_RS") {
							panic!("Environment variable CEF_PATH is not set! This is needed by Browser Window to find CEF's development files. See https://github.com/bamilab/browser-window/tree/master/docs/getting-started for more information.")
						}
					},
					other => panic!("Unable to use CEF_PATH: {}", other)
				}
			},
			Ok(cef_path) => {
				build.include(&cef_path);

				// Link with CEF
				println!("cargo:rustc-link-search={}/libcef_dll_wrapper", &cef_path);
				println!("cargo:rustc-link-search={}/Release", &cef_path);
				if target.contains("msvc") {
					println!("cargo:rustc-link-search={}", &cef_path);
					println!("cargo:rustc-link-search={}/libcef_dll_wrapper/Release", &cef_path);
					println!("cargo:rustc-link-lib=static=libcef_dll_wrapper");
					println!("cargo:rustc-link-lib=dylib={}", "libcef");
				} else {
					// cef_dll_wrapper is a static lib, but for some reason it doesn't
					println!("cargo:rustc-link-lib=static={}", "cef_dll_wrapper");
					println!("cargo:rustc-link-lib=dylib={}", "cef");
				}

				// Add X flags to compiler
				match pkg_config::Config::new().arg("--cflags").arg("--libs").probe("x11") {
					Err(_) => {},	// CEF doesn't always use X...
					Ok( result ) => {
		
						// Includes
						for inc in &result.include_paths {
							build.include( inc );
						}
					}
				}
			}
		}
		
		// Source files
		build
			.file("src/application/cef.cpp")
			.file("src/browser_window/cef.cpp")
			.file("src/cef/bw_handle_map.cpp")
			.file("src/cef/client_handler.cpp")
			.file("src/cef/exception.cpp")
			.file("src/cef/util.cpp")
			.define("BW_CEF", None)
			.cpp(true);
	}



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
		.compile("browser_window_c");

	// Let bindgen generate the bindings
	bgbuilder
		.generate().expect("Unable to generate FFI bindings!")
		.write_to_file( out_path.join("c_bindings.rs") ).expect("Unable to write FFI bindings to file!");

	// Update bindgen backup
	if let Ok(_) = env::var("UPDATE_BINDGEN_BACKUP") {
		fs::copy( out_path.join("c_bindings.rs"), backup_file ).expect("Unable to copy backup file");
	}
}



impl bindgen::callbacks::ParseCallbacks for BwBindgenCallbacks {

	fn item_name(&self, item_name: &str) -> Option<String> {
		Some( "c".to_owned() + item_name )
	}
}