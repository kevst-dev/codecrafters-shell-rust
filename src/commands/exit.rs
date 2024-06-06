use super::errors::CommandError;

use super::ShellCommand;

#[derive(Debug)]
pub struct Exit{
    exit_status: Option<i32>,
}

impl ShellCommand for Exit {
    fn new(args: Vec<String>) -> Result<Self, CommandError> {
        match args.len() {
            0 => Ok(Self { exit_status: None }),
            1 => {
                let exit_status = parse_exit_status(&args[0])?;
                Ok(Self { exit_status })
            }
            _ => Err(CommandError::TooManyArguments {
                command: "exit".to_string(),
                expected: 1,
                found: args.len(),
                args: args.to_vec(),
            }),
        }
    }

    fn run(&self) {
        let exit_status = self.exit_status.unwrap_or(0);
        std::process::exit(exit_status);
    }
}

fn parse_exit_status(arg: &str) -> Result<Option<i32>, CommandError> {
    if arg.is_empty() { return Ok(None); }

    arg.trim().parse::<i32>().map_err(|_| {
        let message = [
            "Para el comando 'exit', se esperaba un valor numerico, ".to_string(),
            format!("pero se recibio: '{}'", arg).to_string(),
        ].join("");
        CommandError::InvalidArguments { message }
    }).map(Some)
}


#[cfg(test)]
mod tests {
    use super::*;

    // ---- -- test Self::new() -- ---- \\

    #[test]
    fn test_new_with_empty_arg() {
        let args = vec!["".to_string()];
        let command = Exit::new(args).unwrap();

        assert_eq!(command.exit_status, None);
    }

    #[test]
    fn test_new_with_one_arg() {
        let options = [
            vec!["0".to_string()],
            vec!["1".to_string()],
            vec!["5".to_string()],
            vec!["10".to_string()],
            vec!["20".to_string()],
            vec!["-1".to_string()],
        ];

        for args in options {
            let command = Exit::new(args.clone()).unwrap();
            let expected = args[0].parse::<i32>().unwrap();

            assert_eq!(command.exit_status, Some(expected));
        }
    }

    #[test]
    fn test_new_with_invalid_num_args() {
        let options = [
            vec!["0".to_string(), "text".to_string()],
            vec!["0".to_string(), "text".to_string(), "2".to_string()],
            vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string()],
        ];

        for args in options {
            let err = Exit::new(args.clone()).unwrap_err();

            let expected_err = CommandError::TooManyArguments {
                command: "exit".to_string(),
                expected: 1,
                found: args.len(),
                args: args,
            };

            assert_eq!(err, expected_err);
        }
    }

    #[test]
    fn test_new_invalid_arg() {
        let input = vec!["hello".to_string()];
        let err = Exit::new(input).unwrap_err();

        assert!(matches!(err, CommandError::InvalidArguments { .. }));
    }
}
