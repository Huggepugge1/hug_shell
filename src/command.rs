#[derive(Debug, PartialEq)]
pub enum CommandType {
    BuiltIn(Builtin),
    External,
    None,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub r#type: CommandType,
}

#[derive(Debug, PartialEq)]
pub enum Builtin {
    Cd,
    Exit,
    Ls,
    Pwd,
}

pub mod builtins {
    use super::Builtin;

    use crate::cd;
    use crate::ls;
    use crate::pwd;

    use std::os::unix::process::ExitStatusExt;
    use std::process::Output;

    #[derive(Debug)]
    pub enum BuiltinExitCode {
        TooManyArguments = 1,
        FileNotFound = 50,
        PermissionDenied = 100,
        UnknownError = 200,
    }

    pub fn is_builtin(name: &str) -> bool {
        match name {
            "cd" => true,
            "exit" => true,
            "ls" => true,
            "pwd" => true,
            _ => false,
        }
    }

    pub fn get(name: &str) -> Builtin {
        match name {
            "cd" => Builtin::Cd,
            "exit" => Builtin::Exit,
            "ls" => Builtin::Ls,
            "pwd" => Builtin::Pwd,
            name => panic!("`{name}` is not a builtin!"),
        }
    }

    pub fn run(builtin: Builtin, args: Vec<String>) -> Output {
        match builtin {
            Builtin::Cd => cd::run(args),
            Builtin::Exit => std::process::exit(0),
            Builtin::Ls => ls::run(args),
            Builtin::Pwd => pwd::run(),
        }
    }

    pub fn handle_builtin_error(output: &mut Output, e: std::io::Error) {
        output.stderr.append(&mut e.to_string().into());
        match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                output.status =
                    std::process::ExitStatus::from_raw(BuiltinExitCode::PermissionDenied as i32);
            }
            std::io::ErrorKind::NotFound => {
                output.status =
                    std::process::ExitStatus::from_raw(BuiltinExitCode::FileNotFound as i32);
            }
            _ => {
                output.status =
                    std::process::ExitStatus::from_raw(BuiltinExitCode::UnknownError as i32);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_builtin() {
            assert_eq!(is_builtin("cd"), true);
            assert_eq!(is_builtin("exit"), true);
            assert_eq!(is_builtin("ls"), true);
            assert_eq!(is_builtin("helloworld"), false);
        }

        #[test]
        fn test_get_builtin() {
            assert_eq!(get("cd"), Builtin::Cd);
            assert_eq!(get("exit"), Builtin::Exit);
            assert_eq!(get("ls"), Builtin::Ls);
        }

        #[test]
        #[should_panic(expected = "`helloworld` is not a builtin!")]
        fn test_get_builtin_panic() {
            get("helloworld");
        }
    }
}

pub mod external {
    use super::Command;

    use std::os::unix::process::ExitStatusExt;
    use std::process::Output;

    enum ExternalExitCode {
        FileNotFound = 50,
        PermissionDenied = 100,
        UnknownError = 200,
    }

    pub fn run(command: Command) -> Output {
        match std::process::Command::new(&command.name)
            .args(&command.args)
            .spawn()
        {
            Ok(child) => child.wait_with_output().unwrap(),
            Err(e) => Output {
                status: match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        std::process::ExitStatus::from_raw(ExternalExitCode::FileNotFound as i32)
                    }
                    std::io::ErrorKind::PermissionDenied => std::process::ExitStatus::from_raw(
                        ExternalExitCode::PermissionDenied as i32,
                    ),
                    _ => std::process::ExitStatus::from_raw(ExternalExitCode::UnknownError as i32),
                },
                stdout: Vec::new(),
                stderr: e.to_string().into(),
            },
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::{Command, CommandType};
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
            let command = Command {
                name: "cat".to_string(),
                args: vec!["Cargo.toml".to_string()],
                r#type: CommandType::External,
            };
            let output = run(command);
            assert_eq!(output.status.success(), true);
        }

        #[test]
        fn test_run_error() {
            let command = Command {
                name: "helloworld".to_string(),
                args: Vec::new(),
                r#type: CommandType::External,
            };
            let output = run(command);
            assert_eq!(
                output.status.signal(),
                Some(ExternalExitCode::FileNotFound as i32)
            );
        }
    }
}
