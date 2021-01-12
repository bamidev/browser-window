use crate::application::*;
use crate::browser::*;

use lazy_static::lazy_static;
use tokio;
use unsafe_send_sync::UnsafeSync;



#[test]
fn simple_async_app() {
	let application = Application::initialize( ApplicationSettings::default() );
	let runtime = application.start();

	runtime.run_async(|app| async move {

		let mut bwb = BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com/".into()) );
		bwb.title("Simple Async Test");
		let bw = bwb.build( app ).await;

		bw.close();
	});
}

#[test]
fn correct_parent_cleanup() {
	let application = Application::initialize( ApplicationSettings::default() );
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