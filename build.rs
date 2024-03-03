use std::env;

fn main() {
	// For the MSVC compiler, it seems that (at least with VS2022) the
	// browser-window-c crate gets compiled with the x86 target. Not sure how to
	// prevent that, but adding the browser-window-c static lib to the linker at
	// least shows a meaningful error message.
	let target = env::var("TARGET").unwrap();
	if target.ends_with("msvc") {
		println!("cargo:rustc-link-lib=static=browser-window-c");

		if target.starts_with("x86_64") {
			println!(
				"cargo:warning=There seems to be a bug in rust/cargo when compiling with MSVC for \
				 x86_64. If compiling for x86_64 doesn't work, try using target \
				 `i686-pc-windows-msvc` instead."
			);
		}
	}

	// Make sure one of the browser frameworks is actually selected.
	if env::var("DOCS_RS").is_err()
		&& !cfg!(feature = "cef")
		&& !cfg!(feature = "webkitgtk")
		&& !cfg!(feature = "edge2")
	{
		panic!(
			"No browser framework has been specified. Enable either feature `webkitgtk`, `cef` or \
			 `edge2`."
		);
	}
}
