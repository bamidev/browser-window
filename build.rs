use std::env;

fn main() {
	// For the MSVC compiler, it seems that sometimes linking errors occur, as a
	// result of compiling browser-window-c for a different architecture then the
	// main package. Adding browser-window-c.lib to the linker manually, at least
	// causes a meaningful error to be shown.
	let target = env::var("TARGET").unwrap();
	if target.ends_with("msvc") {
		println!("cargo:rustc-link-lib=static=browser-window-c");
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
