use structopt::StructOpt;
use mcwr::*;

fn main() {
	let args = Args::from_args();
	let ver = args.version.clone();

	if !is_paper_version(args.version) {
		panic!("Invalid paper version!");
	}
	download_server::paper(ver, args.name).unwrap();
}
