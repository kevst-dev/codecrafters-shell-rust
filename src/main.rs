#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;
use commands::execute_command;

const PROMPT: &str = "$ ";

fn read_command() -> String {
    print!("{PROMPT}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line from stdin");

    input.trim().to_string()
}

fn main() {
    loop {
        let input = read_command();

        let command_and_arg: Vec<&str> = input.split_whitespace().collect();
        let command = command_and_arg[0];
        let args = command_and_arg[1..].to_vec();
        let args = args.iter().map(|s| s.to_string()).collect();

        execute_command(command, args);
    }
}
