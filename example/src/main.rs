use browser_window::*;
use std::process::exit;
use tokio;



fn main() {

	let bw_runtime = Runtime::start();
	//let tokio_runtime = tokio::runtime::Runtime::new().unwrap();

	let exit_code = bw_runtime.spawn( program_logic( bw_runtime.app() ) );

	// Return exit code
	exit( exit_code );
}

async fn program_logic( app: Application ) {
	let x = {
		let bw = BrowserBuilder::new( Source::Html( include_str!("example.html").into() ) )
		.title("Example")
		.width( 800 )
		.height( 600 )
		.minimizable( false )
		.maximizable( false )
		.borders( false )
		.resizable( false )
		.handler(|_, cmd, args| {

			println!("Command \"{}\" invoked!", cmd);
			for i in 0..args.len() {
				println!("\tArg {}: {}", i+1, args[i]);
			}
		})
		.build( app.clone() ).await;
		let bw2 = BrowserBuilder::new( Source::Html( include_str!("example.html").into() ) )
			.title("Example")
			.width( 800 )
			.height( 600 )
			.minimizable( false )
			.maximizable( false )
			.borders( false )
			.resizable( true )
			.parent( &bw )
			.build( app.clone() ).await;
		// Let's fetch the title through Javascript
		match bw.eval_js("document.title").await {
			Err(e) => { eprintln!("Something went wrong with evaluating javascript: {}", e) },
			Ok( cookies ) => {
				eprintln!("This is the window title: {}", cookies);
			}
		}

		// Let's execute some bad code
		// This doesn't work because cookies are not available when using Source::Html.
		match bw.eval_js("document.cookie").await {
			Err(e) => { eprintln!("This javascript error is expected when using CEF: {}", e) },
			Ok( cookies ) => {
				eprintln!("Available cookies: {}", cookies);
			}
		}
		
		bw2
	};

	tokio::time::delay_for( tokio::time::Duration::from_millis(10000) ).await;

	match x.eval_js("document.cookie").await {
		Err(e) => { eprintln!("This javascript error is expected when using CEF: {}", e) },
		Ok( cookies ) => {
			eprintln!("Available cookies: {}", cookies);
		}
	}
}
