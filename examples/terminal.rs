use std::{
	env,
	io::prelude::*,
	process::{exit, Command, Stdio},
};

use browser_window::{application::*, browser::*, prelude::*};
use serde_json;

async fn execute_command(bw: BrowserWindow, line: &str) {
	let working_dir = bw
		.eval_js("working_dir")
		.await
		.expect("Unable to obtain working dir from JavaScript!")
		.to_string_unenclosed()
		.to_string();

	let cmd = if env::consts::OS == "windows" {
		Command::new("cmd")
				.arg("/C")
				.arg( line )
				.current_dir( working_dir )
				.stdout( Stdio::piped() )
				.stderr( Stdio::piped() )
				//.kill_on_drop(true)
				.spawn()
				.expect("Command failed to run!")
	} else {
		Command::new("sh")
				.arg("-c")
				.arg( line )
				//.current_dir( working_dir )
				.stdout( Stdio::piped() )
				.stderr( Stdio::piped() )
				//.kill_on_drop(true)
				.spawn()
				.expect("Command failed to run!")
	};

	// Read the output
	let mut stdout = cmd.stdout.unwrap();
	let mut stderr = cmd.stderr.unwrap();
	let mut buffer: [u8; 1024] = [0xFF; 1024];
	loop {
		let stdout_empty = read_stream(&bw, &mut stdout, &mut buffer, "onOutputReceived");
		let stderr_empty = read_stream(&bw, &mut stderr, &mut buffer, "onErrorOutputReceived");

		if !stdout_empty && !stderr_empty {
			break;
		}
	}

	// Notify the terminal that it can type commands again
	bw.exec_js("onExecutionEnded()");
}

fn read_stream<R>(
	bw: &BrowserWindowHandle, reader: &mut R, buffer: &mut [u8], js_func: &str,
) -> bool
where
	R: Read,
{
	match reader.read(buffer) {
		Err(e) => eprintln!("Command error: {}", e),
		Ok(read) => {
			if read == 0 {
				return false;
			}

			// Convert to string
			let string = String::from_utf8_lossy(&buffer[0..read]);
			// Sanitize string input for JavaScript
			let js_string = serde_json::to_string(&*string).unwrap();

			bw.exec_js(&(js_func.to_owned() + "(" + js_string.as_str() + ")"));
		}
	}

	true
}

fn main() {
	let mut settings = ApplicationSettings::default();
	settings.remote_debugging_port = Some(10000);
	let application = match Application::initialize(&settings) {
		Err(e) => panic!("Unable to initialize application: {}", e),
		Ok(app) => app,
	};
	let runtime = application.start();

	let exit_code = runtime.run_async(|app| async move {
		let working_dir = env::current_dir().unwrap();
		let mut html_file = working_dir.clone();
		html_file.push("examples/resources/terminal.html");

		let mut bwb = BrowserWindowBuilder::new(Source::File(html_file));
		bwb.dev_tools(true);
		bwb.size(800, 600);
		bwb.title("Terminal Example");

		let bw = bwb.build(app).await;

		bw.on_message().register_async(|bw, e| {
			// e.cmd is a &str that lives as long as the closure does, but not as long as
			// the future does.
			let cmd = e.cmd.to_string();
			let args = e.args.clone();

			async move {
				match cmd.as_str() {
					"exec" => {
						// The whole command line is passed one string value.
						let cmd_line = &args[0];

						execute_command(bw, &cmd_line.to_string_unenclosed()).await;
					}
					other => {
						eprintln!("Received unsupported command: {}", other);
					}
				}
			}
		});
		bw.window().set_opacity(224);
		bw.window().show();

		// Initialize the script with our working directory.
		// Make sure that it is initializes whether document has been loaded already or
		// not.
		let working_dir_js = serde_json::to_string(working_dir.to_str().unwrap())
			.expect("Invalid working directory characters!");
		match bw
			.eval_js(format!("initialize({})", &working_dir_js).as_str())
			.await
		{
			Err(e) => eprintln!("Javascript Error: {:?}", e),
			Ok(_) => {}
		};
	});

	// Return exit code
	exit(exit_code);
}
