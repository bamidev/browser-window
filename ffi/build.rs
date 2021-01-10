use std::{
	env,
	path::PathBuf
};



fn main() {

	// If this is being build by docs.rs, don't do anything.
	// docs.rs is not able to compile the C/C++ source files because it doesn't have the win32 and cef header files available in their docker system in which they test-build.
	if let Ok(_) = env::var("DOCS_RS") {
		return
	}

	let target = env::var("TARGET").unwrap();
	let out_path = PathBuf::from( env::var("OUT_DIR").expect("Unable to get output directory for FFI bindings") );

	// Compiler builder
	//let mut cbuilder = cc::Build::new();


	println!("cargo:rustc-link-lib=browser_window_c");
	println!("cargo:rustc-link-search=native=/tmp");

	/**************************************
	 *	C header files for bindgen
	 **************************************/
	let mut bgbuilder = bindgen::Builder::default()
		.clang_arg("-DBW_BINDGEN")
		.header("../c/src/application.h")
		.header("../c/src/browser_window.h")
		.header("../c/src/common.h")
		.header("../c/src/err.h")
		.header("../c/src/string.h")
		.header("../c/src/window.h");

	// Define platform-specific C macros that specify which API's to use,
	//     and/or add necessary platform-specific flags to the compiler
	if target.contains("windows") {
		bgbuilder = bgbuilder
			.clang_arg("-DBW_WIN32");
	}
	else {
		bgbuilder = bgbuilder
			.clang_arg("-DBW_GTK");

		match pkg_config::Config::new().atleast_version("3.0").arg("--cflags").probe("gtk+-3.0") {
			Err(e) => panic!("Unable to find GTK 3 development files: {}", e),
			Ok( lib ) => {

				for inc in &lib.include_paths {
					bgbuilder = bgbuilder.clang_arg( format!("-I{}", inc.as_os_str().to_str().unwrap()) );
				}
			}
		}
	}

	if cfg!(feature = "cef") {

		bgbuilder = bgbuilder.clang_arg("-DBW_CEF");
		
		match env::var("CEF_PATH") {
			Err(e) => {
				match e {
					env::VarError::NotPresent => panic!("Environment variable CEF_PATH is not set! This is needed by Browser Window to find CEF's development files. See https://github.com/bamilab/browser-window/tree/master/docs/getting-started for more information."),
					other => panic!("Unable to use CEF_PATH: {}", other)
				}
			},
			Ok(cef_path) => {

				println!("cargo:rustc-link-search={}/libcef_dll_wrapper", &cef_path);
				println!("cargo:rustc-link-search={}/Release", &cef_path);
				if target.contains("msvc") {
					println!("cargo:rustc-link-search={}", &cef_path);
					println!("cargo:rustc-link-search={}/libcef_dll_wrapper/Release", &cef_path);
					println!("cargo:rustc-link-lib=static=libcef_dll_wrapper");
					println!("cargo:rustc-link-lib=dylib={}", "libcef");
				} else {
					// cef_dll_wrapper is a static lib, but for some reason it doesn't
					println!("cargo:rustc-link-lib={}", "cef_dll_wrapper");
					println!("cargo:rustc-link-lib=dylib={}", "cef");
				}

				// Add X linking flags to compiler
				match pkg_config::Config::new().arg("--cflags").arg("--libs").probe("x11") {
					Err(_) => {},	// CEF doesn't always use X...
					Ok( result ) => {

						// Links
						for lib in &result.libs {
							println!("cargo:rustc-link-lib={}", lib);
						}

						// Link search paths
						for path in &result.link_paths {
							println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
						}
					}
				}
			}
		}
	}

	// Let bindgen generate the bindings
	bgbuilder
		.generate().expect("Unable to generate FFI bindings!")
		.write_to_file( out_path.join("c_bindings.rs") ).expect("Unable to write FFI bindings to file!");

	println!("cargo:rustc-link-lib=stdc++");
}