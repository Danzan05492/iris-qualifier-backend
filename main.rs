use std::process::Command;
use std::io::Write;

fn main(){
	let output = Command::new("echo")
		.arg("Hello world")
		.output()
		.expect("Failed to execute command");

	let mut out = std::io::stdout();
	out.write_all(output.stdout.as_slice())
		.expect("Failed to execute command");
}
