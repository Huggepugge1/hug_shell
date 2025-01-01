use std::io::Write;

use crate::builtin::handle_builtin_error;
use crate::command::{Command, CommandKind, UNKNOWN_ERROR_CODE};
use crate::typesystem::Type;

impl Command {
    pub fn run_external(&mut self) -> Type {
        match &self.kind {
            CommandKind::External { name, args } => {
                match std::process::Command::new(name.value.clone())
                    .args(&args.iter().map(|t| t.run_as_arg()).collect::<Vec<String>>())
                    .stdin(match &self.stdin {
                        Some(_) => std::process::Stdio::piped(),
                        None => std::process::Stdio::inherit(),
                    })
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                {
                    Ok(mut child) => {
                        match &self.stdin {
                            Some(output) => {
                                println!("Output: {:?}", output);
                                let stdin = child.stdin.as_mut().unwrap();
                                match stdin.write_all(&output.to_string().as_bytes()) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        return Type::Error {
                                            message: e.to_string(),
                                            code: UNKNOWN_ERROR_CODE,
                                        }
                                    }
                                }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::builtin::BuiltinExitCode;
    use crate::command::CommandKind;
    use crate::lexer::{Token, TokenKind};
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
        let output = Command::new(CommandKind::External {
            name: Token {
                value: "cat".to_string(),
                kind: TokenKind::Word,
            },
            args: vec![Command::new(CommandKind::String("Cargo.toml".to_string()))],
        })
        .run();
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
        let output = Command::new(CommandKind::External {
            name: Token {
                value: "invalid".to_string(),
                kind: TokenKind::Word,
            },
            args: vec![],
        })
        .run();
        match output {
            Type::Error { code, .. } => {
                assert_eq!(code, BuiltinExitCode::FileNotFound as i32);
            }
            _ => panic!("Expected Type::Error"),
        }
    }

    #[test]
    fn test_run_echo_with_string() {
        let output = Command::new(CommandKind::External {
            name: Token {
                value: "echo".to_string(),
                kind: TokenKind::Word,
            },
            args: vec![Command::new(CommandKind::String(
                "Hello, world!".to_string(),
            ))],
        })
        .run();
        match output {
            Type::Output(o) => {
                assert_eq!(o.status.code().unwrap(), 0);
                assert_eq!(String::from_utf8_lossy(&o.stdout), "Hello, world!\n");
            }
            _ => panic!("Expected Type::Output"),
        }
    }

    #[test]
    fn test_run_echo_with_integer() {
        let output = Command::new(CommandKind::External {
            name: Token {
                value: "echo".to_string(),
                kind: TokenKind::Word,
            },
            args: vec![Command::new(CommandKind::Integer(42))],
        })
        .run();
        match output {
            Type::Output(o) => {
                assert_eq!(o.status.code().unwrap(), 0);
                assert_eq!(String::from_utf8_lossy(&o.stdout), "42\n");
            }
            _ => panic!("Expected Type::Output"),
        }
    }

    #[test]
    fn test_run_echo_with_float() {
        let output = Command::new(CommandKind::External {
            name: Token {
                value: "echo".to_string(),
                kind: TokenKind::Word,
            },
            args: vec![Command::new(CommandKind::Float(3.14))],
        })
        .run();
        match output {
            Type::Output(o) => {
                assert_eq!(o.status.code().unwrap(), 0);
                assert_eq!(String::from_utf8_lossy(&o.stdout), "3.14\n");
            }
            _ => panic!("Expected Type::Output"),
        }
    }

    #[test]
    fn test_run_echo_with_boolean() {
        let output = Command::new(CommandKind::External {
            name: Token {
                value: "echo".to_string(),
                kind: TokenKind::Word,
            },
            args: vec![Command::new(CommandKind::Boolean(true))],
        })
        .run();
        match output {
            Type::Output(o) => {
                assert_eq!(o.status.code().unwrap(), 0);
                assert_eq!(String::from_utf8_lossy(&o.stdout), "true\n");
            }
            _ => panic!("Expected Type::Output"),
        }
    }
}
