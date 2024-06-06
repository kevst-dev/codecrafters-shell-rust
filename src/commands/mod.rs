mod errors;
use errors::CommandError;

mod exit;
use exit::Exit;

mod echo;
use echo::Echo;

pub trait ShellCommand {
    fn new(args: Vec<String>) -> Result<Self, CommandError>
    where
        Self: Sized;

    fn run(&self);
}

pub fn execute_command(command: &str, args: Vec<String>) -> Result<(), CommandError> {
    match command {
        "exit" => Exit::new(args)?.run(),
        "echo" => Echo::new(args)?.run(),
        _ => println!("{}: command not found", command),
    }

    Ok(())
}
