use crate::application::*;
use crate::browser::builder::{BrowserWindowBuilder, Source};

use lazy_static::lazy_static;
use tokio;
use unsafe_send_sync::UnsafeSync;



lazy_static! {
    static ref APPLICATION: UnsafeSync<Application> = UnsafeSync::new( Application::initialize() );
}



#[test]
fn simple_async_app() {
    let runtime = APPLICATION.start();

    runtime.run_async(|app| async move {

        let bw = BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com/".into()) )
            .title("Simple Async Test")
            .build( app ).await;

        bw.close();
    });
}

#[test]
fn simple_threaded_app() {

    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let runtime = APPLICATION.start();
    runtime.run(|_app| {
        let app: ApplicationHandleThreaded = _app.into();

        tokio_runtime.spawn(async move {
            let bw = BrowserWindowBuilder::new( Source::Url("https://www.duckduckgo.com/".into()) )
                .title("Simple Threaded Test")
                .build_threaded( app ).await.unwrap();

            bw.close();
        });
    });
}

#[test]
fn correct_parent_cleanup() {
    let runtime = APPLICATION.start();

    runtime.run_async(|app| async move {

        // First create the parent
        let bw_parent = BrowserWindowBuilder::new(Source::Url("https://www.duckduckgo.com/".into()))
            .title("Simple Parent Cleanup Test")
            .build(app).await;

        // Then a child
        let bw_child = BrowserWindowBuilder::new(Source::Url("https://www.google.com/".into()))
            .title("Simple Child Cleanup Test")
            .parent(&bw_parent)
            .build(app).await;

        // Destroy the parent handle, while a handle of the child still exists
        bw_parent.close();

        // Then close the child handle.
        // This should cleanup the parent as well.
        bw_child.close();
    });
}