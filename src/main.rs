#[allow(unused_imports)]
use std::io::{self, Write};

const PROMPT: &str = "$ ";

fn read_command() -> String {
    print!("{PROMPT}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line from stdin");

    input.trim().to_string()
}

fn process_command(command: &str) {
    match command.trim() {
        _ => println!("{}: command not found", command),
    }
}

fn main() {
    loop {
        let command = read_command();

        process_command(&command);
    }
}
