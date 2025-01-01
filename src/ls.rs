use crate::builtin::{handle_builtin_error, BuiltinExitCode};
use crate::command::Command;
use crate::typesystem::Type;

impl Command {
    fn list_dir(&self, path: &str) -> Type {
        match std::fs::read_dir(path) {
            Ok(entries) => self.list_dir_files(entries),
            Err(e) => handle_builtin_error(e),
        }
    }

    fn list_dir_files(&self, entries: std::fs::ReadDir) -> Type {
        let mut files: Vec<Type> = Vec::new();
        for entry in entries {
            match entry {
                Ok(entry) => files.push(Type::File {
                    path: entry.path(),
                    full_path: false,
                }),
                Err(e) => return handle_builtin_error(e),
            }
        }
        Type::Array(files)
    }

    pub fn run_ls(&self) -> Type {
        let args = self.get_args();
        match args.len() {
            0 => self.list_dir("."),
            1 => self.list_dir(&args[0].run_as_arg()),
            _ => Type::Error {
                message: "Too many arguments".into(),
                code: BuiltinExitCode::TooManyArguments as i32,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use crate::builtin::BuiltinExitCode;
    use crate::command::{Command, CommandKind};
    use crate::typesystem::Type;

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
        let output = Command {
            kind: CommandKind::Builtin {
                builtin: crate::builtin::Builtin::Ls,
                args: Vec::new(),
            },
            stdin: None,
        }
        .run_ls();
        match output {
            Type::Array(files) => {
                assert_eq!(files.len(), 9);
                assert_eq!(files[0].to_string(), "src".blue().to_string());
                assert_eq!(files[1].to_string(), "LICENSE".green().to_string());
                assert_eq!(files[2].to_string(), "Cargo.lock".green().to_string());
                assert_eq!(files[3].to_string(), "Cargo.toml".green().to_string());
                assert_eq!(files[4].to_string(), "target".blue().to_string());
                assert_eq!(files[5].to_string(), "test_dir".blue().to_string());
                assert_eq!(files[6].to_string(), ".git".bright_blue().to_string());
                assert_eq!(files[7].to_string(), "README.md".green().to_string());
                assert_eq!(
                    files[8].to_string(),
                    ".gitignore".bright_green().to_string()
                );
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
        let output = Command::new(CommandKind::Builtin {
            builtin: crate::builtin::Builtin::Ls,
            args: vec![Command::new(CommandKind::String("test_dir".to_string()))],
        })
        .run();
        match output {
            Type::Array(files) => {
                assert_eq!(files.len(), 2);
                assert_eq!(files[0].to_string(), "1.txt".green().to_string());
                assert_eq!(files[1].to_string(), "2.txt".green().to_string());
            }
            _ => panic!("Expected Type::Array, got {:?}", output),
        }
    }

    #[test]
    fn test_run_with_invalid_args() {
        let output = Command::new(CommandKind::Builtin {
            builtin: crate::builtin::Builtin::Ls,
            args: vec![Command::new(CommandKind::String("invalid".to_string()))],
        })
        .run();
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
        let output = Command::new(CommandKind::Builtin {
            builtin: crate::builtin::Builtin::Ls,
            args: vec![
                Command::new(CommandKind::String("/".to_string())),
                Command::new(CommandKind::String("/".to_string())),
            ],
        })
        .run();
        match output {
            Type::Error { code, message } => {
                assert_eq!(code, BuiltinExitCode::TooManyArguments as i32);
                assert_eq!(message, "Too many arguments");
            }
            _ => panic!("Expected Error, got {}", output),
        };
    }
}
