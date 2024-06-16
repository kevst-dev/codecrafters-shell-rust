use super::errors::CommandError;
use super::ShellCommand;

use std::env;

#[derive(Debug)]
pub struct Pwd;

impl ShellCommand for Pwd {
    fn new(_args: Vec<String>) -> Result<Self, CommandError> {
        Ok(Self)
    }

    fn run(&self) -> String {
        let current_dir = env::current_dir().unwrap();

        format!("{}", current_dir.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- -- test Self::new() -- ---- \\

    #[test]
    fn test_pwd_new() {
        let args = vec![];
        let _command = Pwd::new(args).unwrap();

        assert!(true);  // Simplemente verificar que no haya errores
    }

    // ---- -- test Self::run() -- ---- \\
    #[test]
    fn test_pwd_run() {
        let command = Pwd;
        let current_dir = env::current_dir().unwrap();
        let expected_output = current_dir.display().to_string();
        
        assert_eq!(command.run(), expected_output);
    }
}
