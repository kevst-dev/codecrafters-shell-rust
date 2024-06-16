mod errors;
use errors::CommandError;

mod exit;
use exit::Exit;

mod echo;
use echo::Echo;

mod c_type;
use c_type::Type;

mod executable;
use executable::execute_external_command;

mod pwd;
use pwd::Pwd;

mod cd;
use cd::Cd;

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
        "pwd"  => Pwd::new(args)?.run(),
        "cd"   => Cd::new(args)?.run(),
        _ => execute_external_command(command, args),
    };

    if !message.is_empty() {
        println!("{}", message);
    }

    Ok(())
}

pub fn get_available_commands() -> Vec<String> {
    vec![
        "exit".to_string(),
        "echo".to_string(),
        "type".to_string(),
        "pwd".to_string(),
        "cd".to_string(),
    ]
}
