use crate::commands::executable::find_executable_program;

use std::process::Command;

pub fn execute_external_command(command: &str, args: Vec<String>) -> String {
    let posible_binary = find_executable_program(command);

    if let Some(binary_path) = posible_binary {
        Command::new(binary_path)
            .args(&args)
            .status()
            .expect("failed to execute process");

        return String::new()
    }

    format!("{}: command not found", command)
}
