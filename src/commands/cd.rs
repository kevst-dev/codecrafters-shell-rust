use std::env;
use std::path::Path;
use super::errors::CommandError;
use super::ShellCommand;

#[derive(Debug)]
pub struct Cd {
    path: String,
}

impl ShellCommand for Cd {
    fn new(args: Vec<String>) -> Result<Self, CommandError> {
        if args.len() != 1 {
            return Err(CommandError::TooManyArguments {
                command: "cd".to_string(),
                expected: 1,
                found: args.len(),
                args: args.to_vec(),
            });
        }

        let path = args[0].clone();
        Ok(Self { path })
    }

    fn run(&self) -> String {
        if self.path == "~" {
            let home = env::var("HOME").unwrap();
            let path = Path::new(&home);

            env::set_current_dir(&path).unwrap();

            return String::new();
        }

        let path = Path::new(&self.path);

        if path.exists() {
            env::set_current_dir(&path).unwrap();

            String::new()
        } else {
            format!("cd: {}: No such file or directory", self.path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_cd_new() {
        let args = vec!["/usr/local/bin".to_string()];
        let command = Cd::new(args).unwrap();

        assert_eq!(command.path, "/usr/local/bin");
    }

    #[test]
    fn test_cd_run_success() {
        let command = Cd {
            path: "/".to_string(),
        };

        let result = command.run();
        assert!(result.is_empty());

        let current_dir = env::current_dir().unwrap();
        let current_dir_str = current_dir.to_str().unwrap();
        assert_eq!(current_dir_str, "/");
    }

    #[test]
    fn test_cd_run_nonexistent_directory() {
        let command = Cd {
            path: "/nonexistent_directory".to_string()
        };

        let result = command.run();
        let expected = format!(
            "cd: {}: No such file or directory",
            command.path
        );

        assert_eq!(result, expected);
    }
}
