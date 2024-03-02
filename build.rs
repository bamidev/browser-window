use std::env;

fn main() {
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
