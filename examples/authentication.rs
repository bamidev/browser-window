//! This is an example that demonstrates how you can get the session cookie data of a site after logging in.

use browser_window::application::*;
use browser_window::browser::*;
use browser_window::cookie::*;
//use browser_window::prelude::*;



fn main() {
	let application = Application::initialize(&ApplicationSettings::default()).expect("unable to initialize");
	let runtime = application.start();

	runtime.run_async(|app| async move {

		let mut bwb = BrowserWindowBuilder::new(Source::Url("https://github.com/login".to_string()));
		//bwb.dev_tools(false)
		bwb.size( 800, 600 );
		bwb.title("Log in to Github");
		let bw = bwb.build(app).await;

		bw.show();

		let cookie_jar = CookieJar::global();
		for cookie in cookie_jar.iter("/", true) {
			println!("Cookie {}: {}", cookie.name(), cookie.value());
		}
	});
}