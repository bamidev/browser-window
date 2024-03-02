use std::{
	env,
	time::{Duration, SystemTime, UNIX_EPOCH},
};

#[cfg(feature = "threadsafe")]
use tokio;

use crate::{application::*, browser::*, cookie::*, prelude::*};

#[test]
fn tests() {
	let exec_path = env::current_dir()
		.unwrap()
		.join("target/debug/browser-window-se");

	let settings = ApplicationSettings {
		engine_seperate_executable_path: Some(exec_path),
		resource_dir: None,
		remote_debugging_port: None,
	};

	let app = Application::initialize(&settings).expect("unable to initialize application");

	// Instead of marking each test with #[test], there is one actual test that runs
	// all different 'test' functions. This is because the Browser Window
	// application can only be initialized once. Also, because `Application` is not
	// `Send`, we can not use it acros multiple tests because they are ran in
	// parallel.
	async_tests(&app);
	#[cfg(feature = "threadsafe")]
	threaded_tests(&app);
}

#[cfg(feature = "threadsafe")]
fn threaded_tests(application: &Application) {
	let bw_runtime = application.start();

	let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

	// First run our own runtime on the main thread
	bw_runtime.run(|_app| {
		let app = _app.into_threaded();

		// Spawn the main logic into the tokio runtime
		tokio_runtime.spawn(async move {
			// TODO: run tests here...

			app.exit(0);
		});
	});
}

fn async_tests(application: &Application) {
	let runtime = application.start();

	let exit_code = runtime.run_async(|app| async move {
		let _bw = async_basic(app.clone()).await;
		#[cfg(not(feature = "webkitgtk"))]
		async_cookies(app.clone()).await;
		//async_correct_parent_cleanup(app).await;
		app.exit(0);
	});

	assert!(exit_code == 0);
}

async fn async_basic(app: ApplicationHandle) -> BrowserWindow {
	let mut bwb = BrowserWindowBuilder::new(Source::Url("https://www.duckduckgo.com/".into()));
	bwb.title("Basic Async Test");
	return bwb.build(app).await;
}

async fn async_cookies(app: ApplicationHandle) {
	if let Some(mut jar) = app.cookie_jar() {
		let cookie = Cookie::new("name", "value");

		// Store cookies
		jar.store("/", &cookie).await.unwrap_err(); // Should give error
		jar.store("http://localhost/", &cookie).await.unwrap();

		// Delete cookie
		assert!(jar.delete("http://localhost/", "name").await == 1);
		jar.store("http://localhost/", &cookie).await.unwrap();
		assert!(jar.delete_all("name").await == 1);
		jar.store("http://localhost/", &cookie).await.unwrap();
		assert!(jar.clear("http://localhost/").await == 1);
		jar.store("http://localhost/", &cookie).await.unwrap();
		assert!(jar.clear_all().await == 1);

		// Using a wrong url
		{
			let mut iter = jar.iter("/", true);
			assert!(iter.next().await.is_none());
		}

		// Finding our set cookie back
		jar.store("http://localhost/", &cookie).await.unwrap();
		let cookie = jar.find("http://localhost/", "name", true).await.unwrap();
		assert!(cookie.name() == "name");
		assert!(cookie.value() == "value");

		// Finding our set cookie back in another way
		let cookie = jar.find_from_all("name").await.unwrap();
		assert!(cookie.name() == "name");
		assert!(cookie.value() == "value");
	}
}

#[test]
/// Checking if all cookie methods work correctly.
fn cookie() {
	if !cfg!(feature = "webkitgtk") {
		let now = SystemTime::now();

		let mut cookie = Cookie::new("name", "value");

		assert!(cookie.name() == "name");
		assert!(cookie.value() == "value");
		assert!(cookie.domain() == "");
		assert!(cookie.path() == "");
		assert!(cookie.expires() == None);

		cookie
			.make_secure()
			.make_http_only()
			.set_path("/")
			.set_domain("127.0.0.1")
			.set_expires(&now)
			.set_creation_time(&now);

		assert!(cookie.domain() == "127.0.0.1");
		assert!(cookie.path() == "/");
		assert!(
			(now.duration_since(UNIX_EPOCH).unwrap()
				- cookie
					.expires()
					.unwrap()
					.duration_since(UNIX_EPOCH)
					.unwrap()) < Duration::from_millis(1)
		);
		assert!(
			(now.duration_since(UNIX_EPOCH).unwrap()
				- cookie.creation_time().duration_since(UNIX_EPOCH).unwrap())
				< Duration::from_millis(1)
		);
	}
}

/// Closes a parent window before closing its child window, to see if the child
/// window handle still is valid and doesn't cause any memory issues.
async fn async_correct_parent_cleanup(app: ApplicationHandle) {
	// First create the parent
	let mut bwb_parent =
		BrowserWindowBuilder::new(Source::Url("https://www.duckduckgo.com/".into()));
	bwb_parent.title("Parent Window");
	let bw_parent = bwb_parent.build(app.clone()).await;

	// Then a child
	let mut bwb_child = BrowserWindowBuilder::new(Source::Url("https://www.google.com/".into()));
	bwb_child.title("Child Window");
	bwb_child.parent(&bw_parent);
	let _bw_child = bwb_child.build(app.clone()).await;
}
