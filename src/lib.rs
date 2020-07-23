pub mod download_server;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
	pub name: String,

	// flags
	#[structopt(default_value = "1.16.1", long, short)]
	pub version: String,

	#[structopt(default_value = "", long, short)]
	pub r#type: String,
}

pub fn is_paper_version(version: String) -> bool {
	let available_versions = ["1.16.1","1.15.2","1.15.1","1.15","1.14.4","1.14.3","1.14.2","1.14.1","1.14","1.13.2","1.13.1","1.13-pre7","1.13","1.12.2","1.12.1","1.12","1.11.2","1.10.2","1.9.4","1.8.8"];
	available_versions.iter().any(|item| version.contains(item))
}