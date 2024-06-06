mod errors;
use errors::CommandError;

mod exit;
use exit::Exit;

pub trait ShellCommand {
    fn new(args: Vec<String>) -> Result<Self, CommandError>
    where
        Self: Sized;

    fn run(&self);
}

pub fn execute_command(command: &str, args: Vec<String>) {
    match command {
        "exit" => Exit::new(args).unwrap().run(),
        _ => println!("{}: command not found", command),
    }
}
