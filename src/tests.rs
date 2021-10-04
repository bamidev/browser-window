use crate::application::*;
use crate::browser::*;

use std::env;

#[cfg(feature = "threadsafe")]
use tokio;



#[test]
fn tests() {
	let exec_path = env::current_dir().unwrap().join("target/debug/browser-window-se");

	let settings = ApplicationSettings {
		engine_seperate_executable_path: Some(exec_path),
		resource_dir: None
	};

	let app = Application::initialize(&settings).expect("unable to initialize application");

	// Instead of marking each test with #[test], there is one actual test that runs all different 'test' functions.
	// This is because the Browser Window application can only be initialized once.
	// Also, because `Application` is not `Send`, we can not use it acros multiple tests because they are ran in parallel.
	#[cfg(not(feature = "threadsafe"))]
	basic_async_example(&app);
	#[cfg(feature = "threadsafe")]
	basic_threaded_example(&app);
	#[cfg(not(feature = "threadsafe"))]
	correct_parent_cleanup(&app);
}

// A basic example
#[cfg(feature = "threadsafe")]
fn basic_threaded_example(application: &Application) {
	let bw_runtime = application.start();

	let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

	// First run our own runtime on the main thread
	bw_runtime.run(|_app| {
		let app = _app.into_threaded();

		// Spawn the main logic into the tokio runtime
		tokio_runtime.spawn(async move{

		});
	});
}

/// A basic async application.
#[cfg(not(feature = "threadsafe"))]
fn basic_async_example(application: &Application) {
	let runtime = application.start();
	
	runtime.run_async(|app| async move {

		let mut bwb = BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com/".into()) );
		bwb.title("Basic Async Test");
		let bw = bwb.build( app ).await;
		
		bw.close();
	});
}

/// Closes a parent window before closing its child window, to see if the child window handle still is valid and doesn't cause any memory issues.
#[cfg(not(feature = "threadsafe"))]
fn correct_parent_cleanup(application: &Application) {
	let runtime = application.start();

	runtime.run_async(|app| async move {
		
		// First create the parent
		let mut bwb_parent = BrowserWindowBuilder::new(Source::Url("https://www.duckduckgo.com/".into()));
		bwb_parent.title("Parent Window");
		let bw_parent = bwb_parent.build(app).await;

		// Then a child
		let mut bwb_child = BrowserWindowBuilder::new(Source::Url("https://www.google.com/".into()));
		bwb_child.title("Child Window");
		bwb_child.parent(&bw_parent);
		let bw_child = bwb_child.build(app).await;

		// Destroy the parent handle, while a handle of the child still exists
		bw_parent.close();

		// Then close the child handle.
		// This should cleanup the parent as well.
		bw_child.close();
	});
}