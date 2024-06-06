use super::errors::CommandError;
use super::ShellCommand;

#[derive(Debug)]
pub struct Echo {
    text: String,
}

impl ShellCommand for Echo {
    fn new(args: Vec<String>) -> Result<Self, CommandError> {
        let text = args.join(" ");

        Ok(Self { text })
    }

    fn run(&self) -> String {
        format!("{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- -- test Self::new() -- ---- \\

    #[test]
    fn test_new_with_empty_arg() {
        let args = vec!["".to_string()];
        let command = Echo::new(args).unwrap();

        assert_eq!(command.text, "");
    }

    #[test]
    fn test_new_with_one_arg() {
        let options = [
            vec!["hello".to_string()],
            vec!["this".to_string()],
            vec!["name".to_string()],
        ];

        for args in options {
            let command = Echo::new(args.clone()).unwrap();

            assert_eq!(command.text, args.join(" "));
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
            let command = Echo::new(args.clone()).unwrap();

            assert_eq!(command.text, args.join(" "));
        }
    }
}
