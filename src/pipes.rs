use crate::command::{Command, CommandKind};
use crate::typesystem::Type;

pub fn run(command: &mut Command) -> Type {
    match &mut command.kind {
        CommandKind::Pipe {
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

    use crate::lexer::{Token, TokenKind};

    #[test]
    fn test_run() {
        let mut command = Command {
            kind: CommandKind::Pipe {
                source: Box::new(Command {
                    kind: CommandKind::String("Hello, world!".into()),
                    stdin: None,
                }),
                destination: Box::new(Command {
                    kind: CommandKind::External {
                        name: Token {
                            value: "grep".into(),
                            kind: TokenKind::Word,
                        },
                        args: vec![Token {
                            value: "world".into(),
                            kind: TokenKind::Word,
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
            kind: CommandKind::Pipe {
                source: Box::new(Command {
                    kind: CommandKind::Pipe {
                        source: Box::new(Command {
                            kind: CommandKind::String("Hello, world!\n".into()),
                            stdin: None,
                        }),
                        destination: Box::new(Command {
                            kind: CommandKind::External {
                                name: Token {
                                    value: "grep".into(),
                                    kind: TokenKind::Word,
                                },
                                args: vec![Token {
                                    value: "world".into(),
                                    kind: TokenKind::Word,
                                }],
                            },
                            stdin: None,
                        }),
                    },
                    stdin: None,
                }),
                destination: Box::new(Command {
                    kind: CommandKind::External {
                        name: Token {
                            value: "wc".into(),
                            kind: TokenKind::Word,
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
