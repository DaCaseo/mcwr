pub fn start_server(_name: &String) {
	use duct::cmd;
	use std::io::{prelude::*, BufReader, stdout, Write};
	let reader = cmd!("java", "-jar", "server.jar", "nogui").dir("server").reader().unwrap();

	let buf_reader = BufReader::new(reader);
	let mut stdout = stdout();

	for l in buf_reader.lines() {
		let line = l.unwrap() + &String::from("\n");
		stdout.write(line.as_bytes()).unwrap(); // TODO: print each stdin character to stdout to give user feedback when typing
	}
}

pub fn stop_server(_name: &String) {
	use std::io::{prelude::*, Write}; // find a way to keep track of handles
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