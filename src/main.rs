use std::fs;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "better_than_vim", about = "A command-based text editor.")]
struct Cli {
	#[structopt(short, long)]
	debug: bool,

	#[structopt(short, long, default_value = "")]
	prompt: String,

	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf
}

fn main(){
	let args = Cli::from_args();

	if !&args.path.exists() {
		if args.debug {
			println!("Creating file as it doesn't currently exist.");
		}

		fs::write(&args.path, b"");
	}

	let mut content = std::fs::read_to_string(&args.path).expect("could not read file");

	println!("{}", content);
}
