use crate::command::{Command, CommandKind};
use crate::typesystem::Type;

impl Command {
    pub fn run_pipe(&mut self) -> Type {
        match &mut self.kind {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::{Token, TokenKind};

    #[test]
    fn test_run() {
        let output = Command::new(CommandKind::Pipe {
            source: Box::new(Command {
                kind: CommandKind::External {
                    name: Token {
                        value: "echo".to_string(),
                        kind: TokenKind::Word,
                    },
                    args: vec![Command::new(CommandKind::String(
                        "Hello, world!".to_string(),
                    ))],
                },
                stdin: None,
            }),
            destination: Box::new(Command {
                kind: CommandKind::External {
                    name: Token {
                        value: "grep".to_string(),
                        kind: TokenKind::Word,
                    },
                    args: vec![Command::new(CommandKind::String("world".to_string()))],
                },
                stdin: None,
            }),
        })
        .run();
        match output {
            Type::Output(output) => {
                assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");
            }
            _ => panic!("Expected Output, got {}", output),
        }
    }

    #[test]
    fn test_run_with_multiple_pipes() {
        let output = Command::new(CommandKind::Pipe {
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
                            args: vec![Command::new(CommandKind::String("world".into()))],
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
        })
        .run();
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
