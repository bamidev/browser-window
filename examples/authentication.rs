//! This is an example that demonstrates how you can get the session cookie data of a site after logging in.

use browser_window::application::*;
use browser_window::browser::*;
use browser_window::prelude::*;

use std::time::Duration;



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

		let cookie_jar = app.cookie_jar();
		
		// Wait until we moved away from the login page
		while bw.url() == "" || bw.url() == "https://github.com/login" || bw.url() == "https://github.com/session" {
			println!("URL: {}", bw.url());
			app.sleep(Duration::from_millis(10000)).await;
		}

		// Check if logged in
		let logged_in_cookie = cookie_jar.find_from_all("logged_in").await;
		if logged_in_cookie.is_none() || logged_in_cookie.unwrap().value() != "yes" {
			eprintln!("Not logged in.");
			bw.close();
			return;
		}

		// Get session cookie
		let session_cookie = cookie_jar.find_from_all("_gh_sess").await.expect("session cookie not found");
		let session_id = session_cookie.value();
		// You can use this `session_id` to do anything, like scraping user information.

		bw.close();

		eprintln!("Logged in with session ID: {}", session_id);
	});
}