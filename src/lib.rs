pub mod actions;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
	pub action: String,
	pub name: String
}

/* pub fn write_commands_to_terminal() {
	use std::io::{stdout, stdin, BufReader, copy};
	let stdin = stdin();
	let mut stdout = stdout();
	let mut reader = BufReader::new(stdin);

	copy(&mut reader, &mut stdout).unwrap();
} */
