pub fn start_server(_name: &String) {
	use duct::cmd;
	use std::io::{prelude::*, BufReader, stdout, Write};
	std::env::set_current_dir("server").unwrap();
	let reader = cmd!("java", "-jar", "server.jar", "nogui").reader().unwrap();

	let buf_reader = BufReader::new(reader);

	for l in buf_reader.lines() {
		let line = l.unwrap() + &String::from("\n");
		let mut stdout = stdout();
		stdout.write(line.as_bytes()).unwrap(); // print each stdin character to stdout to give user feedback when typing
	}
}

pub fn stop_server(_name: &String) { // use std::env::set_current_dir();
	
}

/* pub fn start_server(name: &String) {
	use std::process::{Command, Stdio};
	use std::io::{Write, Read};
	let child = Command::new("java")
		.arg("-jar")
		.arg("server.jar")
		.arg("nogui")
		.stdout(Stdio::piped())
		.current_dir(std::path::PathBuf::from("/home/dacaseo/Documents/mcwr/server/"))
    	.spawn()
		.expect("Failed to execute command");
	
	let mut stdout_string = String::new();
	child.stdout.expect("failed to open stdout of child").read_to_string(&mut stdout_string).unwrap();
	let mut stdout = std::io::stdout();
	
	stdout.write(&stdout_string.as_bytes()).unwrap();
} */