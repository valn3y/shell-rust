use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Child;
use std::process::Stdio;
use std::process::Command;

// Colors
const KGRN: &str = "\x1B[32m";
const KCYN: &str = "\x1B[36m";
const KRED: &str = "\x1b[31m";
const KBLUE: &str = "\x1b[34m";

fn main() {
	welcome_terminal();

	loop {
		print!("{}{:?}{}$ ", KCYN, env::current_dir().unwrap(),KGRN);
		io::stdout().flush().unwrap();

		// Inputs in terminal
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		let mut commands = input.trim().split(" | ").peekable();
		let mut previous_command = None;

		while let Some(command) = commands.next() {
			let mut parts = command.trim().split_whitespace();
			let command = parts.next().unwrap();
			let args = parts;
			
			match command {
				"cd" => {
					let directory = args.peekable().peek().map_or("/", |x| *x);
					let root = Path::new(directory);
					if let Err(e) = env::set_current_dir(&root) {
						eprintln!("{}", e);
					}
				},
				"exit" => {
					close_terminal();
					return;
				},
				command => {
					let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));
					let stdout = if commands.peek().is_some() {
						Stdio::piped()
					} else {
						Stdio::inherit()
					};

					let output = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();
				
					match output {
						Ok(output) => { previous_command = Some(output); },
						Err(e) => {
							previous_command = None;
							eprintln!("{}", e);
						}
					}
				}
			}
		}

		if let Some(mut final_command) = previous_command {
			final_command.wait().unwrap();
		}
	}
}

fn welcome_terminal() {
	print!("{}", KBLUE);
    println!("***********************************************************");
    println!("***********************************************************");
    println!("                    TERMINAL IN RUST                     \n");
    println!("                          /\\");
    println!("                         |  |");
    println!("                         |  |");
    println!("                        .\'  \'.");
    println!("                        |    |");
    println!("                        |    |");
    println!("                        | /\\ |");
    println!("                      .' |__|'.");
    println!("                      |  |  |  |");
    println!("                     .'  |  |  '.");
    println!(r"                /\    |  \__/  |    /\");
    println!("               |  |  |   |  |   |  |  |");
    println!("           /|  |  |,-\\   |  |   /-,|  |  |\\");
    println!("           ||  |,-'   |  |  |  |   '-,|  ||");
    println!("           ||-'       |  |  |  |       '-||");
    println!("|\\     _,-'           |  |  |  |           '-,_     /|");
    println!("||  ,-'   _           |  |  |  |               '-,  ||");
    println!("||-'    =(*)=         |  |  |  |                  '-||");
    println!("||                    |  \\  /  |                    ||");
    println!(r"|\________....--------\\  ||   /--------....________/|");
    println!("                      /|  ||  |\\");
    println!("                     / |  ||  | \\");
    println!("                    /  |  \\/  |  \\");
    println!("                   /   |      |   \\   ");
    println!("                  /   .|      |.   \\");
    println!(r"                .'|_./ |      | \._|'.");
    println!("               /    _.-|||  |||-._    \\");
    println!(r"               \__.-'  \\||/\\||/'-.__/ ");
	println!("\n");
    println!("***********************************************************");
    println!("***********************************************************");
    println!("\n");
}

fn close_terminal() {
	println!("\n");
	println!("{}Good Bye!", KRED);
}