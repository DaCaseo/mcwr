pub mod actions;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
	pub action: String,
	pub name: String
}
