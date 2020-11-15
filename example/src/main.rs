use browser_window::*;
use std::process::exit;
use tokio;



fn main() {

	let bw_runtime = Runtime::start();

	let tk_runtime = tokio::runtime::Runtime::new().unwrap();
	let exit_code = bw_runtime.run( |app| {
		tk_runtime.spawn( program_logic( app.into() ) );
	} );

	// Return exit code
	exit( exit_code );
}

async fn program_logic( app: ApplicationThreaded ) {

	let x = {
		let bw = BrowserBuilder::new( Source::Html( include_str!("example.html").into() ) )
		.title("Example")
		.width( 800 )
		.height( 600 )
		.minimizable( false )
		.borders( false )
		.resizable( false )
		.handler(|_, cmd, args| {

			println!("Command \"{}\" invoked!", cmd);
			for i in 0..args.len() {
				println!("\tArg {}: {}", i+1, args[i]);
			}
		})
		.build_threaded( app.clone() ).await.unwrap();

		let bw2 = BrowserBuilder::new( Source::Html( include_str!("example.html").into() ) )
			.title("Example")
			.width( 800 )
			.height( 600 )
			.minimizable( false )
			.borders( false )
			.resizable( true )
			.parent( &bw )
			.build_threaded( app.clone() ).await.unwrap();

		// Let's fetch the title through Javascript
		match bw.eval_js("document.title").await.unwrap() {
			Err(e) => { eprintln!("Something went wrong with evaluating javascript: {}", e) },
			Ok( cookies ) => {
				eprintln!("This is the window title: {}", cookies);
			}
		}

		// Let's execute some bad code
		// This doesn't work because cookies are not available when using Source::Html.
		match bw.eval_js("document.cookie").await.unwrap() {
			Err(e) => { eprintln!("This javascript error is expected when using CEF: {}", e) },
			Ok( cookies ) => {
				eprintln!("Available cookies: {}", cookies);
			}
		}

		bw2
	};

	let number = x.delegate_async(|_| async {
		eprintln!("Before panic");
		panic!("Panic!");
		eprintln!("After panic");

		return 14
	}).await.unwrap();
	eprintln!("Delegate result: {}", number);

	tokio::time::delay_for( tokio::time::Duration::from_millis(20000) ).await;

	match x.eval_js("document.cookie").await.unwrap() {
		Err(e) => { eprintln!("This javascript error is expected when using CEF: {}", e) },
		Ok( cookies ) => {
			eprintln!("Available cookies: {}", cookies);
		}
	}

	eprintln!("END");
}
