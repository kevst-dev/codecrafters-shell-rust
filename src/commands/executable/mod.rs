use std::env;

mod run;
pub use run::execute_external_command;

pub fn find_executable_program(binary_name: &str) -> Option<String> {
    let path_env = env::var("PATH").unwrap();
    let dir_paths = path_env.split(':').collect::<Vec<&str>>();

    for dir in dir_paths {
        let binary_path = format!("{}/{}", dir, binary_name);
        let posible_file = std::fs::metadata(&binary_path);

        if posible_file.is_ok() {
            return Some(binary_path);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_executable_program() {
        let frequent_executable = [
            "ls",
            "cat",
        ];

        for executable in frequent_executable {
            let binary_path = find_executable_program(executable);

            assert!(binary_path.is_some());
        }
    }

    #[test]
    fn test_find_executable_program_not_found() {
        let not_executable = [
            "hello",
            "world",
            "pineapple",
            "strawberry",
            "lorem",
        ];

        for executable in not_executable {
            let binary_path = find_executable_program(executable);

            assert!(binary_path.is_none());
        }
    }
}
