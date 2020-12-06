use std::fs;
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

		fs::write(&args.path, b"").expect("could not write file");
	}

	let mut content = std::fs::read_to_string(&args.path).expect("could not read file");

	while prompt(&args, &mut content) {};
}

fn prompt(args: &Cli, content: &mut String) -> bool {
	//print!("{}", args.prompt); // For some reason this isn't working rn

	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	//let line = read!("{}\n");

	return run_command(&args, line, content);
}

fn run_command(args: &Cli, command: String, content: &mut String) -> bool {
	match command.trim() {
		"." => {
			println!("Apologies, this isn't supported yet");
		},
		"a" => {
			loop {
				let mut line = String::new();
				std::io::stdin().read_line(&mut line).unwrap();

				if line.as_str().trim() == "." {
					if args.debug {
						println!("Exiting append mode.");
					}

					break;
				}

				content.push_str(line.as_str());

				if args.debug {
					println!("Appending {}", line.trim());
				}
			}
		},
		"w" => {
			fs::write(&args.path, content.as_bytes()).expect("could not write file");
		},
		"q" => {
			return false;
		},
		_ => {
			println!("?");
		}
	}

	return true;
}
