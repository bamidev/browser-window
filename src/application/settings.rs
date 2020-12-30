use std::{
	env,
	path::PathBuf
};



pub struct Settings {
	pub cef_resource_dir: Option<PathBuf>
}



/*impl Settings {

	pub fn default_resource_path() -> PathBuf {
		let mut path = env::current_exe().unwrap();
		path.pop();

		#[cfg(debug_assertions)]
		{ path.pop(); path.pop(); }

		path.push("resources");

		path
	}
}*/

impl Default for Settings {
	fn default() -> Self {
		Self {
			cef_resource_dir: None
		}
	}
}