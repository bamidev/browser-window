extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::{
	env,
	ffi::{OsStr, OsString},
	fs,
	path::{Path, PathBuf},
	process::Command,
};

#[derive(Debug)]
struct BwBindgenCallbacks {}

/// Prints all compiler commands to rerun if any file has changed within the
/// given directory or a subdictory thereof.
fn rerun_if_directory_changed<P>(_path: P)
where
	P: Into<PathBuf>,
{
	let path: PathBuf = _path.into();

	let dir_iterator = match fs::read_dir(&path) {
		Err(e) => panic!("Unable to read directory: {}", e),
		Ok(iterator) => iterator,
	};

	for sub_path in dir_iterator {
		match sub_path {
			Err(e) => panic!(
				"Unable to read directory entry for dir {}: {}",
				path.as_os_str().to_str().unwrap(),
				e
			),
			Ok(entry) =>
				if entry.path().is_dir() {
					rerun_if_directory_changed(entry.path());
				} else {
					println!(
						"cargo:rerun-if-changed={}",
						entry.path().as_os_str().to_str().unwrap()
					);
				},
		}
	}
}

fn to_executable_args<'a>(args: &'a [OsString]) -> Vec<&'a OsStr> {
	let ignore_args: [OsString; 1] = ["-fPIC".into()];

	let mut new_args = Vec::with_capacity(args.len());

	for arg in args {
		if !(ignore_args.contains(arg)) {
			new_args.push(arg.as_os_str());
		}
	}

	new_args
}

fn to_executable_compiler_command(
	tool: cc::Tool, library_args: &[OsString], source_file: &str, out_file: &Path,
) -> Command {
	let mut cmd = Command::new(tool.path());
	cmd.args(to_executable_args(tool.args()));

	if tool.is_like_gnu() || tool.is_like_clang() {
		let extra_args: [OsString; 5] = [
			"-o".into(),
			out_file.as_os_str().into(),
			source_file.into(),
			"-lcef_dll_wrapper".into(),
			"-lcef".into(),
		];
		cmd.args(&extra_args);
	} else if tool.is_like_msvc() {
		let extra_args: [OsString; 5] = [
			format!("/Fe:{}", out_file.to_str().unwrap()).into(),
			source_file.into(),
			"/link".into(),
			"libcef_dll_wrapper.lib".into(),
			"libcef.lib".into(),
		];
		cmd.args(&extra_args);
	} else {
		panic!("Compiler type not recognized for seperate executable.")
	}

	cmd.args(library_args);

	return cmd;
}

fn main() {
	println!("cargo:rerun-if-env-changed=CEF_PATH");
	rerun_if_directory_changed("src");

	let target = env::var("TARGET").unwrap();
	let out_path = PathBuf::from(
		env::var("OUT_DIR").expect("Unable to get output directory for C/C++ code base crate"),
	);
	let target_dir = out_path
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.parent()
		.unwrap();
	let backup_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap() + "/bindgen_backup.rs");

	// Workaround for docs.rs
	// Docs.rs is not able to compile the C/C++ source files because it doesn't have
	// the win32 and cef header files available in their docker system in which they
	// test-build.
	if let Ok(_) = env::var("DOCS_RS") {
		fs::copy(&backup_file, out_path.join("c_bindings.rs"))
			.expect("Unable to copy backup c bindings");
		return;
	}

	let mut build = cc::Build::new();
	let mut build_se = cc::Build::new(); // For seperate executable
	let std_flag = if cfg!(feature = "cef") {
		if target.contains("msvc") {
			"/std:c++17"
		} else {
			"-std=c++17"
		}
	} else {
		if target.contains("msvc") {
			"/std:c11"
		} else {
			"-std=c11"
		}
	};

	if target.contains("windows") {
		build.define("BW_WINDOWS", None);
		build_se.define("BW_WINDOWS", None);
	}
	if target.contains("msvc") {
		build.flag("/MT");
	}

	/**************************************
	 *	C header files for bindgen
	 ************************ */
	let mut bgbuilder = bindgen::Builder::default()
		.parse_callbacks(Box::new(BwBindgenCallbacks {}))
		.clang_arg("-DBW_BINDGEN")
		.header("src/application.h")
		.header("src/browser_window.h")
		.header("src/cookie.h")
		.header("src/common.h")
		.header("src/err.h")
		.header("src/event.h")
		.header("src/string.h")
		.header("src/window.h");

	// When opting for using GTK:
	/*if cfg!(feature = "gtk") {
		bgbuilder = bgbuilder.clang_arg("-DBW_GTK");

		// GTK source files
		build
			.file("src/application/gtk.c")
			.file("src/window/gtk.c")
			.define("BW_GTK", None);
		build_se.define("BW_GTK", None);

		match pkg_config::Config::new()
			.atleast_version("3.0")
			.arg("--cflags")
			.probe("gtk+-3.0")
		{
			Err(e) => panic!("Unable to find GTK 3 development files: {}", e),
			Ok(lib) => {
				// Manually add GTK includes to compiler
				for inc in &lib.include_paths {
					build.include(inc);
				}
			}
		}
	}*/

	/*******************************************
	 *	The Browser Engine (CEF3) source files *
	 ****************************************** */
	if cfg!(feature = "cef") {
		bgbuilder = bgbuilder.clang_arg("-DBW_CEF").clang_arg("-DBW_CEF_WINDOW");

		build.flag_if_supported("-Wno-unused-parameter"); // CEF's header files produce a lot of unused parameters warnings.
		build_se.flag_if_supported("-Wno-unused-parameter");
		let mut build_se_lib_args = Vec::<OsString>::new();

		match env::var("CEF_PATH") {
			Err(e) => {
				match e {
					env::VarError::NotPresent => {
						// Disable checking CEF_PATH for the docs.rs compiler, it is not on their
						// system anyway.
						if let Err(_) = env::var("DOCS_RS") {
							panic!(
								"Environment variable CEF_PATH is not set! This is needed by Browser Window to find CEF's development files. See https://github.com/bamilab/browser-window/tree/master/docs/getting-started for more information."
							)
						}
					}
					other => panic!("Unable to use CEF_PATH: {}", other),
				}
			}
			Ok(cef_path) => {
				build.include(&cef_path);
				build_se.include(&cef_path);

				// Link with CEF
				println!("cargo:rustc-link-search={}/libcef_dll_wrapper", &cef_path);
				println!("cargo:rustc-link-search={}/Release", &cef_path);
				if target.contains("msvc") {
					build_se.flag("/MT");

					println!("cargo:rustc-link-search={}", &cef_path);
					println!(
						"cargo:rustc-link-search={}/libcef_dll_wrapper/Release",
						&cef_path
					);
					println!("cargo:rustc-link-lib=static=libcef_dll_wrapper");
					println!("cargo:rustc-link-lib=static=libcef");
					println!("cargo:rustc-link-lib=dylib=libcef");

					build_se_lib_args.push(format!("/LIBPATH:{}", &cef_path).into());
					build_se_lib_args
						.push(format!("/LIBPATH:{}/libcef_dll_wrapper", &cef_path).into());
					build_se_lib_args
						.push(format!("/LIBPATH:{}/libcef_dll_wrapper/Release", &cef_path).into());
					build_se_lib_args.push(format!("/LIBPATH:{}/Release", &cef_path).into());
				} else {
					println!("cargo:rustc-link-lib=static=cef_dll_wrapper");
					println!("cargo:rustc-link-lib=static=cef");
					println!("cargo:rustc-link-lib=dylib=cef");

					build_se_lib_args.push(format!("-L{}/libcef_dll_wrapper", &cef_path).into());
					build_se_lib_args.push(format!("-L{}/Release", &cef_path).into());

                    let pkg_deps = [
                        "alsa",
                        "cups",
                        "dbus-1",
                        "expat",
                        "gtk+-3.0",
                        "libdrm",
                        "nss",
                        "gbm",
                        "x11",
                        "x11-xcb",
                        "xcomposite",
                        "xdamage",
                        "xext",
                        "xfixes",
                        "xkbcommon",
                        "xrandr",
                    ];
                    for pkg_name in pkg_deps {
                        let result = pkg_config::Config::new().probe(pkg_name).expect(&format!("Unable to find {}", pkg_name));

                        if pkg_name == "x11" {
                            // When X11 is used, browser-window-c uses a bit of x11 code as well
                            for inc in &result.include_paths {
                              build.include(inc);
                              build_se.include(inc);
                            }
                        }
                    }
				}

			}
		}

		// Source files
		build
			.file("src/application/cef.cpp")
			.file("src/application/cef_window.cpp")
			.file("src/browser_window/cef.cpp")
			.file("src/cookie/cef.cpp")
			.file("src/cef/bw_handle_map.cpp")
			.file("src/cef/client_handler.cpp")
			.file("src/cef/exception.cpp")
			.file("src/cef/util.cpp")
			.file("src/window/cef.cpp")
			.define("BW_CEF", None)
			.define("BW_CEF_WINDOW", None)
			.flag(std_flag)
			.cpp(true);

		// Build the seperate executable and copy it to target/debug (or target/release)
		build_se
			.define("BW_CEF", None)
			.define("BW_CEF_WINDOW", None)
			.cpp(true)
			.flag(std_flag);
		let se_comp = build_se.get_compiler();

		for var in se_comp.env() {
			env::set_var(&var.0, &var.1);
		}

		let se_file = if !target.contains("windows") {
			out_path.join("browser-window-se")
		} else {
			out_path.join("browser-window-se.exe")
		};
		let mut se_cmd = to_executable_compiler_command(
			se_comp,
			&*build_se_lib_args,
			"src/cef/seperate_executable.cpp",
			&se_file,
		);

		let status = se_cmd
			.status()
			.expect("unable to get status of seperate executable compiler");
		assert!(
			status.code().unwrap() == 0,
			"Seperate executable compiler failed with error code {}.",
			status.code().unwrap()
		);

		if !target.contains("windows") {
			fs::copy(se_file, target_dir.join("browser-window-se"))
				.expect("unable to copy seperate executable");
		} else {
			fs::copy(se_file, target_dir.join("browser-window-se.exe"))
				.expect("unable to copy seperate executable");
		}
	}
	/****************************************
	 * Microsoft Edge WebView2 source files
	 ************************************** */
	else if cfg!(feature = "edge2") {
		bgbuilder = bgbuilder.clang_arg("-DBW_WIN32").clang_arg("-DBW_EDGE2");

		// Win32 API
		build
			.file("src/win32.c")
			.file("src/application/win32.c")
			.file("src/window/win32.c")
			.define("BW_WIN32", None)
			.define("BW_EDGE2", None)
			.define("_CRT_SECURE_NO_WARNINGS", None); // Disable sprintf_s warnings. sprintf_s tends to cause segfaults anyway...

		// Add the MinGW header files and libraries when available
		if Path::new("/usr/share/mingw-w64/include/").exists() {
			//bgbuilder = bgbuilder.clang_arg("-I/lib/gcc/x86_64-w64-mingw32/12-win32/include");
			build.include(PathBuf::from(
				env::var("CARGO_MANIFEST_DIR").unwrap() + "/win32/include",
			));

			bgbuilder = bgbuilder
				//.clang_arg("--target=x86_64-pc-windows-gnu")
				.clang_arg("-I/usr/lib/gcc/x86_64-w64-mingw32/12-win32/include");
		}

		bgbuilder = bgbuilder.clang_args(&["-DBW_WIN32", "-DBW_EDGE2"]);

		build
			.define("BW_WIN32", None)
			.define("BW_EDGE2", None)
			//.include(include_dir)
			.file("src/application/edge2.c")
			.file("src/application/win32.c")
			.file("src/browser_window/edge2.c")
			.file("src/window/win32.c")
			.file("src/cookie/unsupported.c")
			.file("src/win32.c");
	}
	else {
		build.file("src/application/other.c");
	}

	/**************************************
	 *	All other source files
	 ************************ */
	build
		.file("src/application/common.c")
		.file("src/browser_window/common.c")
		.file("src/err.c")
		.file("src/event.c")
		.file("src/string.c")
		.file("src/window/common.c")
		//.flag(std_flag)
		.compile("browser-window-c");

	// Let bindgen generate the bindings
	bgbuilder
		.generate()
		.expect("Unable to generate FFI bindings!")
		.write_to_file(out_path.join("c_bindings.rs"))
		.expect("Unable to write FFI bindings to file!");

	// Update bindgen backup
	if let Ok(_) = env::var("UPDATE_BINDGEN_BACKUP") {
		fs::copy(out_path.join("c_bindings.rs"), backup_file).expect("Unable to copy backup file");
	}
}

impl bindgen::callbacks::ParseCallbacks for BwBindgenCallbacks {
	fn item_name(&self, item_name: &str) -> Option<String> { Some("c".to_owned() + item_name) }
}
