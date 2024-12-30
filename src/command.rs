use super::lexer::Token;

use crate::pipes;
use crate::redirect;

#[derive(Debug)]
pub struct Command {
    pub command: CommandType,
    pub stdin: Option<crate::typesystem::Type>,
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.command == other.command
    }
}

impl Command {
    pub const NONE: Command = Command {
        command: CommandType::None,
        stdin: None,
    };

    pub fn run(&mut self) -> crate::typesystem::Type {
        match &self.command {
            CommandType::Builtin { .. } => builtins::run(self),
            CommandType::External { .. } => external::run(self),
            CommandType::String(s) => crate::typesystem::Type::String(s.clone()),

            CommandType::Redirect { .. } => redirect::run(self),
            CommandType::Pipe { .. } => pipes::run(self),

            CommandType::None => crate::typesystem::Type::Null,

            CommandType::Error(e) => crate::typesystem::Type::Error {
                message: e.clone(),
                code: 1,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Builtin {
        builtin: Builtin,
        args: Vec<Token>,
    },
    External {
        name: Token,
        args: Vec<Token>,
    },
    String(String),

    Redirect {
        source: Box<Command>,
        destination: Box<Command>,
    },
    Pipe {
        source: Box<Command>,
        destination: Box<Command>,
    },

    None,
    Error(String),
}

#[derive(Debug, PartialEq)]
pub enum Builtin {
    Cd,
    Exit,
    Ls,
    Pwd,
}

pub mod builtins {
    use super::{Builtin, Command, CommandType};

    use crate::typesystem::Type;

    use crate::cd;
    use crate::ls;
    use crate::pwd;

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

    pub fn run(command: &Command) -> Type {
        match &command.command {
            CommandType::Builtin { builtin, args } => match builtin {
                Builtin::Cd => cd::run(args),
                Builtin::Exit => std::process::exit(0),
                Builtin::Ls => ls::run(args),
                Builtin::Pwd => pwd::run(args),
            },
            _ => Type::Null,
        }
    }

    pub fn handle_builtin_error(e: std::io::Error) -> Type {
        Type::Error {
            message: e.to_string(),
            code: match e.kind() {
                std::io::ErrorKind::NotFound => BuiltinExitCode::FileNotFound as i32,
                std::io::ErrorKind::PermissionDenied => BuiltinExitCode::PermissionDenied as i32,
                _ => BuiltinExitCode::UnknownError as i32,
            },
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
    use super::{Command, CommandType};

    use std::io::Write;

    use crate::command::builtins::handle_builtin_error;
    use crate::typesystem::Type;

    pub fn run(command: &Command) -> Type {
        match &command.command {
            CommandType::External { name, args } => {
                match std::process::Command::new(name.value.clone())
                    .args(
                        &args
                            .iter()
                            .map(|t| t.value.clone())
                            .collect::<Vec<String>>(),
                    )
                    .stdin(match &command.stdin {
                        Some(Type::File { file, .. }) => {
                            std::process::Stdio::from(file.try_clone().unwrap())
                        }
                        _ => std::process::Stdio::piped(),
                    })
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                {
                    Ok(mut child) => {
                        match &command.stdin {
                            Some(Type::Output(o)) => {
                                let stdin = child.stdin.as_mut().unwrap();
                                stdin.write_all(&o.stdout).unwrap();
                            }
                            Some(t) => {
                                let stdin = child.stdin.as_mut().unwrap();
                                stdin.write_all(&t.to_string().as_bytes()).unwrap();
                            }
                            _ => (),
                        }
                        Type::Output(child.wait_with_output().unwrap())
                    }
                    Err(e) => handle_builtin_error(e),
                }
            }
            _ => unreachable!(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::CommandType;
        use super::*;

        use crate::command::builtins::BuiltinExitCode;
        use crate::lexer::{Token, TokenType};
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
            let command = Command {
                command: CommandType::External {
                    name: Token {
                        value: "cat".to_string(),
                        r#type: TokenType::Word,
                    },
                    args: vec![Token {
                        value: "Cargo.toml".to_string(),
                        r#type: TokenType::Word,
                    }],
                },
                stdin: None,
            };
            let output = run(&command);
            match output {
                Type::Output(o) => {
                    assert_eq!(o.status.code().unwrap(), 0);
                    assert_eq!(
                        String::from_utf8_lossy(&o.stdout),
                        include_str!("../Cargo.toml")
                    );
                }
                _ => panic!("Expected Type::Output"),
            }
        }

        #[test]
        fn test_run_with_invalid_command() {
            let command = Command {
                command: CommandType::External {
                    name: Token {
                        value: "invalid".to_string(),
                        r#type: TokenType::Word,
                    },
                    args: vec![],
                },
                stdin: None,
            };
            let output = run(&command);
            match output {
                Type::Error { code, .. } => {
                    assert_eq!(code, BuiltinExitCode::FileNotFound as i32);
                }
                _ => panic!("Expected Type::Error"),
            }
        }
    }
}
