use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
//use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
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

		File::create(&args.path).expect("could not write file");
	}

	let mut file = OpenOptions::new()
		.read(true)
		.write(true)
		.open(&args.path)
		.expect("could not read file");

	let mut buffer = Vec::<String>::new();

	BufReader::new(file)
		.lines()
		.for_each(|l| buffer.push(l.expect("could not read line")) );

	let mut addr: usize = buffer.len() - 1;

	if args.debug {
		println!("{:?}", buffer);
	}

	while prompt(&args, &mut addr, &mut buffer) {};
}

fn prompt(args: &Cli, addr: &mut usize, buffer: &mut Vec<String>) -> bool {
	//print!("{}", args.prompt); // For some reason this isn't working rn

	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();

	return run_command(&args, line, addr, buffer);
}

fn run_command(args: &Cli, command: String, addr: &mut usize, buffer: &mut Vec<String>) -> bool {
	match command.trim() {
		"." => {
			println!("Apologies, this isn't supported yet");
		},
		"a" => {
			loop {
				let mut line = String::new();
				std::io::stdin().read_line(&mut line).unwrap();
				line = line.trim().to_string();

				if line.as_str() == "." {
					if args.debug {
						println!("Exiting append mode.");
					}

					break;
				}

				if args.debug {
					println!("Appending {}", line);
				}

				buffer.insert((*addr as usize) + 1 , line);
				*addr += 1;
			}

			if args.debug {
				println!("{:?}", buffer);
			}
		},
		"w" => {
			//fs::write(&args.path, content.as_bytes()).expect("could not write file");
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
