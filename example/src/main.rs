use browser_window::*;
use std::process::exit;
use tokio;



fn main() {

	let app = Application::new();

	// Start the tokio runtime and run our actual main function on it
	let runtime = tokio::runtime::Runtime::new().unwrap();
	runtime.spawn( program_logic( app.clone().into() ) );

	let exit_code = app.run();

	// Return exit code
	exit( exit_code );
}

async fn program_logic( app: ApplicationAsync ) {

	BrowserWindowBuilder::new( Source::Url( "https://www.google.com".to_owned() ) )
		.title("Example".to_owned())
		.width( 80 )
		.height( 600 )
		.spawn_async( &app ).await;
}
