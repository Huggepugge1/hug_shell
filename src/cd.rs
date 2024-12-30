use homedir::my_home;

use std::path::PathBuf;

use crate::command::builtins::{handle_builtin_error, BuiltinExitCode};
use crate::lexer::Token;
use crate::typesystem::Type;

enum CdExitCode {
    HomeDirNotFound = 25,
}

fn set_dir(path: PathBuf) -> Type {
    match std::env::set_current_dir(path) {
        Ok(_) => Type::Null,
        Err(e) => handle_builtin_error(e),
    }
}

fn set_home_dir() -> Type {
    set_dir(match my_home() {
        Ok(Some(path)) => path,
        Ok(None) => {
            return Type::Error {
                message: "Could not find home directory".into(),
                code: CdExitCode::HomeDirNotFound as i32,
            }
        }
        Err(e) => {
            return Type::Error {
                message: e.to_string(),
                code: BuiltinExitCode::UnknownError as i32,
            }
        }
    })
}

pub fn run(args: &Vec<Token>) -> Type {
    if args.is_empty() {
        set_home_dir()
    } else if args.len() > 1 {
        Type::Error {
            message: "Too many arguments".into(),
            code: BuiltinExitCode::TooManyArguments as i32,
        }
    } else {
        set_dir(args[0].value.clone().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use homedir::my_home;

    #[test]
    fn test_run() {
        let output = run(&Vec::new());
        match output {
            Type::Null => {
                assert_eq!(
                    my_home().unwrap().unwrap(),
                    std::env::current_dir().unwrap()
                );
            }
            _ => panic!("Expected Null, got {}", output),
        }
    }

    #[test]
    fn test_run_with_arg() {
        let output = run(&vec![Token {
            value: "/".to_string(),
            r#type: crate::lexer::TokenType::Word,
        }]);
        match output {
            Type::Null => {
                assert_eq!(
                    std::env::current_dir().unwrap().to_string_lossy(),
                    "/".to_string()
                );
            }
            _ => panic!("Expected Null, got {}", output),
        }
    }

    #[test]
    fn test_run_with_invalid_arg() {
        let output = run(&vec![Token {
            value: "invalid".to_string(),
            r#type: crate::lexer::TokenType::Word,
        }]);
        match output {
            Type::Error { code, .. } => {
                assert_eq!(code, BuiltinExitCode::FileNotFound as i32);
            }
            _ => panic!("Expected Error, got {}", output),
        };
    }

    #[test]
    fn test_run_with_too_many_args() {
        let output = run(&vec![
            Token {
                value: "test_dir".to_string(),
                r#type: crate::lexer::TokenType::String,
            },
            Token {
                value: "invalid".to_string(),
                r#type: crate::lexer::TokenType::String,
            },
        ]);
        match output {
            Type::Error { code, .. } => {
                assert_eq!(code, BuiltinExitCode::TooManyArguments as i32);
            }
            _ => panic!("Expected Error, got {}", output),
        }
    }
}
