//! This is an example that demonstrates how you can get the session cookie data
//! of a site after logging in.

use std::time::Duration;

use browser_window::{application::*, browser::*, event::EventExt, prelude::*};

fn main() {
	let application =
		Application::initialize(&ApplicationSettings::default()).expect("unable to initialize");
	let runtime = application.start();

	runtime.run_async(|app| async move {
		let mut bwb =
			BrowserWindowBuilder::new(Source::Url("https://github.com/login".to_string()));
		//bwb.dev_tools(false)
		bwb.size(800, 600);
		bwb.title("Log in to Github");
		let bw = bwb.build(&app).await;

		bw.on_navigation_start().register(|_h, _arg| {
			println!("on_navigation_start");
		});
		bw.on_navigation_end().register(|_h, result| {
			println!("on_navigation_end {:?}", result);
		});
		bw.on_page_title_changed().register(|_h, result| {
			println!("on_page_title_changed {:?}", result);
		});


		bw.show();

		let cookie_jar = app.cookie_jar().expect("cookies not supported");

		// Wait until we moved away from the login page
		while bw.url() == ""
			|| bw.url() == "https://github.com/login"
			|| bw.url() == "https://github.com/session"
		{
			app.sleep(Duration::from_millis(100)).await;
		}

		// Check if logged in
		let logged_in_cookie = cookie_jar.find_from_all("logged_in").await;
		if logged_in_cookie.is_none() || logged_in_cookie.unwrap().value() != "yes" {
			eprintln!("Not logged in.");
		} else {
			// Get session cookie
			let session_cookie = cookie_jar
				.find_from_all("_gh_sess")
				.await
				.expect("session cookie not found");
			let session_id = session_cookie.value();
			// You can use this `session_id` to do anything, like scraping user information.

			eprintln!("Logged in with session ID: {}", session_id);
		}
	});
}
