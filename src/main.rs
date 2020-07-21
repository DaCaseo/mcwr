use structopt::StructOpt;
use mcwr::*;

fn main() {
	let args = Args::from_args();

	match &args.action[..] {
		"start" => actions::start_server(&args.name),
		"stop" => actions::stop_server(&args.name),
		_ => println!("\"{}\" is not a command", args.name)
	}
}
