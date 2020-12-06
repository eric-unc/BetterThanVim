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

struct State<'a> {
	args: &'a Cli,
	buffer: &'a mut Vec<String>,
	addr: &'a mut usize,
	yank_buffer: &'a mut String
}

impl<'a> State<'a> {
	pub fn new(args: &'a Cli, buffer: &'a mut Vec<String>, addr: &'a mut usize, yank_buffer: &'a mut String) -> State<'a> {
		State { args: args, buffer: buffer, addr: addr, yank_buffer: yank_buffer }
	}
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

	if buffer.len() == 0 {
		buffer.push(String::from(""));
	}

	let mut addr: usize = buffer.len() - 1;

	let mut yank_buffer = String::from("");

	let mut state = State::new(&args, &mut buffer, &mut addr, &mut yank_buffer);

	while prompt(&mut state) {};
}

fn prompt(state: &mut State) -> bool {
	//print!("{}", args.prompt); // For some reason this isn't working rn

	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();

	return run_command(state, line);
}

fn run_command(state: &mut State, command: String) -> bool {
	let ln = command.trim();

	match ln {
		"." => {
			println!("{}", state.buffer[*state.addr]);
		},
		"r" => {
			println!("{}", *state.addr);
		},
		"^" => {
			*state.addr = 0;
		},
		"$" => {
			*state.addr = state.buffer.len() - 1;
		},
		"+" => {
			if *state.addr + 1 >= state.buffer.len() {
				println!("?");
			} else {
				*state.addr += 1;
			}
		},
		"-" => {
			if *state.addr == 0 {
				println!("?");
			} else {
				*state.addr += 1;
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

				state.buffer.insert((*state.addr as usize) + 1 , line);
				*state.addr += 1;
			}
		},
		"c" => {
			let mut line = String::new();
			std::io::stdin().read_line(&mut line).unwrap();
			line = line.trim().to_string();

			if line.as_str() != "." {
				state.buffer[*state.addr] = line;
			}
		},
		"d" => {
			if (*state.buffer).len() != 1 {
				state.buffer.remove(*state.addr);

				if *state.addr == state.buffer.len() {
					*state.addr -= 1;
				}
			} else {
				if state.buffer[0].as_str() != "" {
					state.buffer[0] = String::from("");
				} else {
					println!("?");
				}
			}
		},
		"y" => {
			*state.yank_buffer = state.buffer[*state.addr].clone();
		},
		"x" => {
			if (*state.buffer).len() != 1 {
				*state.yank_buffer = state.buffer[*state.addr].clone();
				state.buffer.remove(*state.addr);

				if *state.addr == state.buffer.len() {
					*state.addr -= 1;
				}
			} else {
				if state.buffer[0].as_str() != "" {
					*state.yank_buffer = state.buffer[*state.addr].clone();
					state.buffer[0] = String::from("");
				} else {
					println!("?");
				}
			}
		},
		"p" => {
			if state.yank_buffer.as_str() != "" {
				state.buffer.insert((*state.addr as usize) + 1 , state.yank_buffer.clone());
				*state.addr += 1;
			} else {
				println!("?");
			}
		},
		"w" => {
			let mut content = String::from("");

			for line in state.buffer.iter() {
				content.push_str((line.to_owned() + "\n").as_str());
			}

			fs::write(&state.args.path, content.as_bytes()).expect("could not write file");
		},
		"q" => {
			return false;
		},
		_ => {
			// TODO: I can't figure out how to make this static.
			let n = Regex::new("^(\\d+)$").unwrap();
			let dot_n = Regex::new("^.(\\d+)$").unwrap();

			if n.is_match(ln) {
				let line_num = ln.parse::<usize>().unwrap();

				if line_num >= state.buffer.len() {
					println!("?");
				} else {
					*state.addr = line_num;
				}
			} else if dot_n.is_match(ln) {
				let cap = dot_n.captures(ln).unwrap();

				println!("{}", &cap[1]);

				let line_num = (&cap[1]).parse::<usize>().unwrap();

				if line_num >= state.buffer.len() {
					println!("?");
				} else {
					println!("{}", state.buffer[line_num]);
				}
			} else {
				println!("?");
			}
		}
	}

	return true;
}
