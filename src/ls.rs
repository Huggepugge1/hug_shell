use crate::command::builtins::{handle_builtin_error, BuiltinExitCode};
use crate::typesystem::Type;

fn list_dir(path: &str) -> Type {
    match std::fs::read_dir(path) {
        Ok(entries) => list_dir_files(entries),
        Err(e) => handle_builtin_error(e),
    }
}

fn list_dir_files(entries: std::fs::ReadDir) -> Type {
    let mut files: Vec<Type> = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => files.push(Type::File(
                std::fs::File::open(&entry.path()).unwrap(),
                entry.path(),
            )),
            Err(e) => return handle_builtin_error(e),
        }
    }
    Type::Array(files)
}

pub fn run(args: Vec<String>) -> Type {
    if args.len() > 1 {
        Type::Error {
            message: "Too many arguments".into(),
            code: BuiltinExitCode::TooManyArguments as i32,
        }
    } else if args.is_empty() {
        list_dir(".")
    } else {
        list_dir(&args[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        // Set the current directory to the root of the project
        std::env::set_current_dir(
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        )
        .unwrap();
        let output = run(Vec::new());
        match output {
            Type::Array(files) => {
                assert_eq!(files.len(), 8);
                assert_eq!(files[0].to_string(), "src".to_string());
                assert_eq!(files[1].to_string(), "LICENSE".to_string());
                assert_eq!(files[2].to_string(), "Cargo.lock".to_string());
                assert_eq!(files[3].to_string(), "Cargo.toml".to_string());
                assert_eq!(files[4].to_string(), "target".to_string());
                assert_eq!(files[5].to_string(), "test_dir".to_string());
                assert_eq!(files[6].to_string(), ".git".to_string());
                assert_eq!(files[7].to_string(), "README.md".to_string());
                assert_eq!(files[8].to_string(), ".gitignore".to_string());
            }
            _ => panic!("Expected Type::Array"),
        }
    }

    #[test]
    fn test_run_with_args() {
        // Set the current directory to the root of the project
        std::env::set_current_dir(
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        )
        .unwrap();
        let output = run(vec!["test_dir".to_string()]);
        match output {
            Type::Array(files) => {
                assert_eq!(files.len(), 2);
                assert_eq!(files[0].to_string(), "1.txt".to_string());
                assert_eq!(files[1].to_string(), "2.txt".to_string());
            }
            _ => panic!("Expected Type::Array"),
        }
    }

    #[test]
    fn test_run_with_invalid_args() {
        let output = run(vec!["invalid".to_string()]);
        match output {
            Type::Error { code, message } => {
                assert_eq!(code, BuiltinExitCode::FileNotFound as i32);
                assert_eq!(message, "No such file or directory (os error 2)");
            }
            _ => panic!("Expected Error, got {}", output),
        };
    }

    #[test]
    fn test_run_with_too_many_args() {
        let output = run(vec![
            "too".to_string(),
            "many".to_string(),
            "args".to_string(),
        ]);
        match output {
            Type::Error { code, message } => {
                assert_eq!(code, BuiltinExitCode::TooManyArguments as i32);
                assert_eq!(message, "Too many arguments");
            }
            _ => panic!("Expected Error, got {}", output),
        };
    }
}
