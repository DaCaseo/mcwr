use structopt::StructOpt;
use mcwr::*;
use tmux_interface::TmuxInterface;

fn main() {
	let args = Args::from_args();
	let tmux = TmuxInterface::new();

	match &args.action[..] {
		"start" => actions::start_server(&args, tmux),
		"stop" => actions::stop_server(&args, tmux),
		_ => println!("\"{}\" is not a command", args.name)
	}
}
