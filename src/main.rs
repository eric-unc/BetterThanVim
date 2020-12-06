use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;

use regex::Regex;
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
		File::create(&args.path).expect("could not write file");
	}

	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.open(&args.path)
		.expect("could not read file");

	let mut buffer = Vec::<String>::new();

	BufReader::new(file)
		.lines()
		.for_each(|l| buffer.push(l.expect("could not read line")) );

	let mut addr: usize = buffer.len() - 1;

	while prompt(&args, &mut addr, &mut buffer) {};
}

fn prompt(args: &Cli, addr: &mut usize, buffer: &mut Vec<String>) -> bool {
	//print!("{}", args.prompt); // For some reason this isn't working rn

	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();

	return run_command(&args, line, addr, buffer);
}

fn run_command(args: &Cli, command: String, addr: &mut usize, buffer: &mut Vec<String>) -> bool {
	let ln = command.trim();

	match ln {
		"." => {
			println!("{}", buffer[*addr]);
		},
		"^" => {
			*addr = 0;
		},
		"$" => {
			*addr = buffer.len() - 1;
		},
		"+" => {
			if *addr + 1 >= buffer.len() {
				println!("?");
			} else {
				*addr += 1;
			}
		},
		"-" => {
			if *addr - 1 < 0 {
				println!("?");
			} else {
				*addr += 1;
			}
		},
		"a" => {
			loop {
				let mut line = String::new();
				std::io::stdin().read_line(&mut line).unwrap();
				line = line.trim().to_string();

				if line.as_str() == "." {
					break;
				}

				buffer.insert((*addr as usize) + 1 , line);
				*addr += 1;
			}
		},
		"w" => {
			let mut content = String::from("");

			for line in buffer {
				content.push_str((line.to_owned() + "\n").as_str());
			}

			fs::write(&args.path, content.as_bytes()).expect("could not write file");
		},
		"q" => {
			return false;
		},
		_ => {
			// TODO: I can't figure out how to make this static.
			let n = Regex::new("^(\\d+)$").unwrap();

			if n.is_match(ln) {
				let line_num = ln.parse::<usize>().unwrap();

				if line_num >= buffer.len() {
					println!("?");
				} else {
					*addr = line_num;
				}
			} else {
				println!("?");
			}
		}
	}

	return true;
}
