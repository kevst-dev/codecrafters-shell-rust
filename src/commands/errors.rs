use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CommandError {
    #[error("{message:?}")]
    InvalidArguments {
        message: String,
    },

    #[error("Para el comando '{command:?}', se esperaban {expected} argumentos, pero se encontraron {found}. Args: {args:?}")]
    TooManyArguments {
        command: String,
        expected: usize,
        found: usize,
        args: Vec<String>,
    },
}
