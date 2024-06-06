mod errors;
use errors::CommandError;

mod exit;
use exit::Exit;

mod echo;
use echo::Echo;

mod c_type;
use c_type::Type;

pub trait ShellCommand {
    fn new(args: Vec<String>) -> Result<Self, CommandError>
    where
        Self: Sized;

    fn run(&self) -> String;
}

pub fn execute_command(command: &str, args: Vec<String>) -> Result<(), CommandError> {
    let message = match command {
        "exit" => Exit::new(args)?.run(),
        "echo" => Echo::new(args)?.run(),
        "type" => Type::new(args)?.run(),
        _ => format!("{}: command not found", command),
    };

    println!("{}", message);

    Ok(())
}

pub fn get_available_commands() -> Vec<String> {
    vec![
        "exit".to_string(),
        "echo".to_string(),
        "type".to_string(),
    ]
}
