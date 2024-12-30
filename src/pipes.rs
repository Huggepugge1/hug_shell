use crate::command::{Command, CommandType};
use crate::typesystem::Type;

pub fn run(command: &mut Command) -> Type {
    match &mut command.command {
        CommandType::Pipe {
            source,
            destination,
        } => {
            let source_output = source.run();
            destination.stdin = Some(source_output);
            destination.run()
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use colored::Colorize;

    use crate::lexer::{Token, TokenType};

    #[test]
    fn test_run() {
        let mut command = Command {
            command: CommandType::Pipe {
                source: Box::new(Command {
                    command: CommandType::String("Hello, world!".into()),
                    stdin: None,
                }),
                destination: Box::new(Command {
                    command: CommandType::External {
                        name: Token {
                            value: "grep".into(),
                            r#type: TokenType::Word,
                        },
                        args: vec![Token {
                            value: "world".into(),
                            r#type: TokenType::Word,
                        }],
                    },
                    stdin: None,
                }),
            },
            stdin: None,
        };
        let output = run(&mut command);
        match output {
            Type::Output(output) => {
                assert_eq!(
                    String::from_utf8(output.stdout).unwrap(),
                    "\"Hello, world!\"".green().to_string() + "\n"
                );
            }
            _ => panic!("Expected Output, got {}", output),
        }
    }

    #[test]
    fn test_run_with_multiple_pipes() {
        let mut command = Command {
            command: CommandType::Pipe {
                source: Box::new(Command {
                    command: CommandType::Pipe {
                        source: Box::new(Command {
                            command: CommandType::String("Hello, world!\n".into()),
                            stdin: None,
                        }),
                        destination: Box::new(Command {
                            command: CommandType::External {
                                name: Token {
                                    value: "grep".into(),
                                    r#type: TokenType::Word,
                                },
                                args: vec![Token {
                                    value: "world".into(),
                                    r#type: TokenType::Word,
                                }],
                            },
                            stdin: None,
                        }),
                    },
                    stdin: None,
                }),
                destination: Box::new(Command {
                    command: CommandType::External {
                        name: Token {
                            value: "wc".into(),
                            r#type: TokenType::Word,
                        },
                        args: vec![],
                    },
                    stdin: None,
                }),
            },
            stdin: None,
        };
        let output = run(&mut command);
        match output {
            Type::Output(output) => {
                assert_eq!(
                    String::from_utf8(output.stdout).unwrap(),
                    "      1       2      20\n"
                );
            }
            _ => panic!("Expected Output, got {}", output),
        }
    }
}
