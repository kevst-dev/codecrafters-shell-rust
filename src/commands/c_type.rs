use super::errors::CommandError;
use super::{ShellCommand};
use super::get_available_commands;
use super::executable::find_executable_program;

#[derive(Debug, PartialEq)]
pub struct Type {
    available_commands: Vec<String>,
    input_command: String,
}

impl ShellCommand for Type {
    fn new(args: Vec<String>) -> Result<Self, CommandError> {
        let available_commands = get_available_commands();
        let input_command = args.join(" ");

        Ok(Self {
            available_commands,
            input_command,
        })
    }

    fn run(&self) -> String {
        // Busqueda de comandos de la Shell
        for command in &self.available_commands {
            if command == &self.input_command {
                return format!("{} is a shell builtin", command);
            }
        }

        // Busqueda de ejecutables
        let binary_path = find_executable_program(&self.input_command);
        if binary_path.is_some() {
            return format!(
                "{} is {}",
                self.input_command,
                binary_path.unwrap()
            );
        }

        format!("{}: not found", self.input_command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- -- test Self::new() -- ---- \\

    #[test]
    fn test_new_with_empty_arg() {
        let args = vec!["".to_string()];
        let command = Type::new(args).unwrap();

        let expected_type = Type {
            available_commands: get_available_commands(),
            input_command: "".to_string(),
        };

        assert_eq!(command, expected_type);
    }

    #[test]
    fn test_new_with_one_arg() {
        let options = [
            vec!["hello".to_string()],
            vec!["this".to_string()],
            vec!["name".to_string()],
        ];

        for args in options {
            let command = Type::new(args.clone()).unwrap();

            let expected_type = Type {
                available_commands: get_available_commands(),
                input_command: args.join(" "),
            };

            assert_eq!(command, expected_type);
        }
    }

    #[test]
    fn test_new_with_valid_args() {
        let options = [
            vec!["hello".to_string(), "world".to_string()],
            vec!["pineapple".to_string(), "strawberry".to_string()],
            vec!["lorem".to_string(), "ip".to_string(), "sum".to_string()],
        ];

        for args in options {
            let command = Type::new(args.clone()).unwrap();

            let expected_type = Type {
                available_commands: get_available_commands(),
                input_command: args.join(" "),
            };

            assert_eq!(command, expected_type);
        }
    }

    // ---- -- test Self::run() -- ---- \\

    #[test]
    fn test_evaluate_input_with_valid_command_shell() {
        let commands = ["type", "echo", "exit"];

        for command in commands {
            let command = Type {
                available_commands: get_available_commands(),
                input_command: command.to_string(),
            };
            let expected_msg = format!("{} is a shell builtin", command.input_command);

            let message = command.run();
            assert_eq!(message, expected_msg);
        }
    }

    #[test]
    fn test_evaluate_input_with_valid_executable_programs_in_path() {
        let frequent_executable = [
            "ls",
            "cat",
        ];

        for executable in frequent_executable {
            let command = Type {
                available_commands: get_available_commands(),
                input_command: executable.to_string(),
            };
            let start_msg = format!("{} is", executable);

            let message = command.run();

            assert!(message.starts_with(&start_msg));
            assert!(message.ends_with(executable));
        }
    }

    #[test]
    fn test_evaluate_input_with_invalid_command() {
        let command = Type {
            available_commands: get_available_commands(),
            input_command: "hello".to_string(),
        };
        let expected_msg = format!("hello not found");

        let message = command.run();
        assert_eq!(message, expected_msg);
    }
}
