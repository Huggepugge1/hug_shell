use crate::builtin::BuiltinExt;
use crate::command::{Command, CommandKind};
use crate::lexer::{Token, TokenKind};

pub struct Parser<'a> {
    tokens: std::iter::Peekable<std::slice::Iter<'a, Token>>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: std::iter::Peekable<std::slice::Iter<'a, Token>>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();

        while let Some(t) = self.tokens.peek() {
            if commands.len() > 0 {
                match t.kind {
                    TokenKind::SemiColon => {
                        self.tokens.next();
                        if self.tokens.peek().is_none() {
                            break;
                        }
                    }
                    _ => {
                        return vec![Command {
                            kind: CommandKind::Error(format!("Expected `;`, found `{}`", t.value)),
                            stdin: None,
                        }]
                    }
                }
            }
            let command = self.parse_statement();
            match command.kind {
                CommandKind::Error(_) => {
                    commands.push(command);
                    break;
                }
                _ => commands.push(command),
            }
        }
        commands
    }

    fn parse_statement(&mut self) -> Command {
        let mut command = self.parse_expression();

        while let Some(token) = self.tokens.peek() {
            command = match token.kind {
                TokenKind::GreaterThan | TokenKind::Pipe => self.parse_binary(command),
                TokenKind::SemiColon => {
                    break;
                }
                _ => Command {
                    kind: CommandKind::Error("Unexpected token".to_string()),
                    stdin: None,
                },
            };
        }
        command
    }

    // Expression is a maximum of one binary operator
    fn parse_expression(&mut self) -> Command {
        if let Some(token) = self.tokens.peek() {
            match token.kind {
                TokenKind::Word => self.parse_word(),
                TokenKind::String => self.parse_string(),
                TokenKind::Boolean => self.parse_boolean(),
                TokenKind::Integer => self.parse_integer(),
                TokenKind::Float => self.parse_float(),
                TokenKind::SemiColon => Command {
                    kind: CommandKind::None,
                    stdin: None,
                },
                _ => Command {
                    kind: CommandKind::Error("Unexpected token".to_string()),
                    stdin: None,
                },
            }
        } else {
            Command {
                kind: CommandKind::Error("Unexpected end of input".to_string()),
                stdin: None,
            }
        }
    }

    fn parse_word(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        let builtin = if token.value.is_builtin() {
            Some(token.value.get_builtin())
        } else {
            None
        };

        let args = self.parse_args();

        if let Some(builtin) = builtin {
            Command {
                kind: CommandKind::Builtin { builtin, args },
                stdin: None,
            }
        } else {
            Command {
                kind: CommandKind::External {
                    name: token.clone(),
                    args,
                },
                stdin: None,
            }
        }
    }

    fn parse_args(&mut self) -> Vec<Command> {
        let mut args = Vec::new();
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                TokenKind::Word => {
                    args.push(Command {
                        kind: CommandKind::String(token.value.clone()),
                        stdin: None,
                    });
                    self.tokens.next();
                }
                TokenKind::String => {
                    args.push(Command {
                        kind: CommandKind::String(token.value.clone()),
                        stdin: None,
                    });
                    self.tokens.next();
                }
                TokenKind::Boolean => {
                    args.push(Command {
                        kind: CommandKind::Boolean(token.value.parse().unwrap()),
                        stdin: None,
                    });
                    self.tokens.next();
                }
                TokenKind::Integer => {
                    args.push(Command {
                        kind: CommandKind::Integer(token.value.parse().unwrap()),
                        stdin: None,
                    });
                    self.tokens.next();
                }
                TokenKind::Float => {
                    args.push(Command {
                        kind: CommandKind::Float(token.value.parse().unwrap()),
                        stdin: None,
                    });
                    self.tokens.next();
                }
                _ => break,
            }
        }
        args
    }

    fn parse_string(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        Command {
            kind: CommandKind::String(token.value.clone()),
            stdin: None,
        }
    }

    fn parse_boolean(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        Command {
            kind: CommandKind::Boolean(token.value.parse().unwrap()),
            stdin: None,
        }
    }

    fn parse_integer(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        Command {
            kind: CommandKind::Integer(token.value.parse().unwrap()),
            stdin: None,
        }
    }

    fn parse_float(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        Command {
            kind: CommandKind::Float(token.value.parse().unwrap()),
            stdin: None,
        }
    }

    fn parse_binary(&mut self, command: Command) -> Command {
        match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::GreaterThan => Command {
                    kind: CommandKind::Redirect {
                        source: Box::new(command),
                        destination: Box::new(self.parse_expression()),
                    },
                    stdin: None,
                },
                TokenKind::Pipe => Command {
                    kind: CommandKind::Pipe {
                        source: Box::new(command),
                        destination: Box::new(self.parse_expression()),
                    },
                    stdin: None,
                },
                _ => Command {
                    kind: CommandKind::Error("Unexpected token".to_string()),
                    stdin: None,
                },
            },
            None => Command {
                kind: CommandKind::Error("Unexpected end of input".to_string()),
                stdin: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::Builtin;

    #[test]
    fn test_parse_cd() {
        let tokens = vec![Token {
            value: "cd".to_string(),
            kind: TokenKind::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_cd_with_one_arg() {
        let tokens = vec![
            Token {
                value: "cd".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "/home".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(
                    *args,
                    vec![Command::new(CommandKind::String("/home".to_string()))]
                );
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_cd_with_multiple_args() {
        let tokens = vec![
            Token {
                value: "cd".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "/home".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "arg2".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(
                    *args,
                    vec![
                        Command::new(CommandKind::String("/home".to_string())),
                        Command::new(CommandKind::String("arg2".to_string()))
                    ]
                );
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_exit() {
        let tokens = vec![Token {
            value: "exit".to_string(),
            kind: TokenKind::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Exit);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_exit_with_one_arg() {
        let tokens = vec![
            Token {
                value: "exit".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "1".to_string(),
                kind: TokenKind::Integer,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Exit);
                assert_eq!(*args, vec![Command::new(CommandKind::Integer(1))]);
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_exit_with_multiple_args() {
        let tokens = vec![
            Token {
                value: "exit".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "1".to_string(),
                kind: TokenKind::Integer,
            },
            Token {
                value: "2".to_string(),
                kind: TokenKind::Integer,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Exit);
                assert_eq!(
                    *args,
                    vec![
                        Command::new(CommandKind::Integer(1)),
                        Command::new(CommandKind::Integer(2))
                    ]
                );
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_ls() {
        let tokens = vec![Token {
            value: "ls".to_string(),
            kind: TokenKind::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_ls_with_one_arg() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "arg1".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(
                    *args,
                    vec![Command::new(CommandKind::String("arg1".to_string()))]
                );
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_ls_with_multiple_args() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "arg1".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "arg2".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(
                    *args,
                    vec![
                        Command::new(CommandKind::String("arg1".to_string())),
                        Command::new(CommandKind::String("arg2".to_string()))
                    ]
                );
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token {
            value: "hello".to_string(),
            kind: TokenKind::String,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::String(ref value) => {
                assert_eq!(value, "hello");
            }
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_parse_boolean() {
        let tokens = vec![Token {
            value: "true".to_string(),
            kind: TokenKind::Boolean,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Boolean(value) => {
                assert_eq!(value, true);
            }
            _ => panic!("Expected Boolean"),
        }

        let tokens = vec![Token {
            value: "false".to_string(),
            kind: TokenKind::Boolean,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Boolean(value) => {
                assert_eq!(value, false);
            }
            _ => panic!("Expected Boolean"),
        }
    }

    #[test]
    fn test_parse_external() {
        let tokens = vec![Token {
            value: "helloworld".to_string(),
            kind: TokenKind::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::External { ref name, ref args } => {
                assert_eq!(
                    *name,
                    Token {
                        value: "helloworld".to_string(),
                        kind: TokenKind::Word,
                    }
                );
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected External"),
        }

        let tokens = vec![
            Token {
                value: "helloworld".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "arg1".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "arg2".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::External { ref name, ref args } => {
                assert_eq!(
                    *name,
                    Token {
                        value: "helloworld".to_string(),
                        kind: TokenKind::Word,
                    }
                );
                assert_eq!(
                    *args,
                    vec![
                        Command::new(CommandKind::String("arg1".to_string())),
                        Command::new(CommandKind::String("arg2".to_string()))
                    ]
                );
            }
            _ => panic!("Expected External"),
        }
    }

    #[test]
    fn test_parse_empty() {
        let tokens = Vec::new();
        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_parse_redirect() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: ">".to_string(),
                kind: TokenKind::GreaterThan,
            },
            Token {
                value: "output.txt".to_string(),
                kind: TokenKind::String,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Redirect {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        kind: CommandKind::Builtin {
                            builtin: Builtin::Ls,
                            args: Vec::new()
                        },
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        kind: CommandKind::String("output.txt".to_string()),
                        stdin: None,
                    }
                );
            }
            _ => panic!("Expected Redirect"),
        }
    }

    #[test]
    fn test_parse_redirect_from_string() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::String,
            },
            Token {
                value: ">".to_string(),
                kind: TokenKind::GreaterThan,
            },
            Token {
                value: "output.txt".to_string(),
                kind: TokenKind::String,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Redirect {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        kind: CommandKind::String("ls".to_string()),
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        kind: CommandKind::String("output.txt".to_string()),
                        stdin: None,
                    }
                );
            }
            _ => panic!("Expected Redirect"),
        }
    }

    #[test]
    fn test_parse_pipe() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: "|".to_string(),
                kind: TokenKind::Pipe,
            },
            Token {
                value: "grep".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Pipe {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        kind: CommandKind::Builtin {
                            builtin: Builtin::Ls,
                            args: Vec::new()
                        },
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        kind: CommandKind::External {
                            name: Token {
                                value: "grep".to_string(),
                                kind: TokenKind::Word
                            },
                            args: Vec::new()
                        },
                        stdin: None,
                    }
                );
            }
            _ => panic!("Expected Pipe"),
        }
    }

    #[test]
    fn test_parse_pipe_from_string() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::String,
            },
            Token {
                value: "|".to_string(),
                kind: TokenKind::Pipe,
            },
            Token {
                value: "grep".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Pipe {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        kind: CommandKind::String("ls".to_string()),
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        kind: CommandKind::External {
                            name: Token {
                                value: "grep".to_string(),
                                kind: TokenKind::Word
                            },
                            args: Vec::new()
                        },
                        stdin: None,
                    }
                );
            }
            _ => panic!("Expected Pipe"),
        }
    }

    #[test]
    fn test_parse_multiple_commands() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: ";".to_string(),
                kind: TokenKind::SemiColon,
            },
            Token {
                value: "cd".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 2);

        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }

        match commands[1].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_three_commands() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: ";".to_string(),
                kind: TokenKind::SemiColon,
            },
            Token {
                value: "cd".to_string(),
                kind: TokenKind::Word,
            },
            Token {
                value: ";".to_string(),
                kind: TokenKind::SemiColon,
            },
            Token {
                value: "pwd".to_string(),
                kind: TokenKind::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 3);

        match commands[0].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }

        match commands[1].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }

        match commands[2].kind {
            CommandKind::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Pwd);
                assert_eq!(*args, Vec::<Command>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn parse_only_semicolon() {
        let tokens = vec![Token {
            value: ";".to_string(),
            kind: TokenKind::SemiColon,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 1);
        match commands[0].kind {
            CommandKind::None => (),
            _ => panic!("Expected None"),
        }
    }

    #[test]
    fn parse_multiple_semicolons() {
        let tokens = vec![
            Token {
                value: ";".to_string(),
                kind: TokenKind::SemiColon,
            },
            Token {
                value: ";".to_string(),
                kind: TokenKind::SemiColon,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 2);
        match commands[0].kind {
            CommandKind::None => (),
            _ => panic!("Expected None"),
        }
        match commands[1].kind {
            CommandKind::None => (),
            _ => panic!("Expected None"),
        }
    }

    #[test]
    fn parse_integer() {
        let tokens = vec![Token {
            value: "123".to_string(),
            kind: TokenKind::Integer,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Integer(value) => {
                assert_eq!(value, 123);
            }
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn parse_float() {
        let tokens = vec![Token {
            value: "3.14".to_string(),
            kind: TokenKind::Float,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].kind {
            CommandKind::Float(value) => {
                assert_eq!(value, 3.14);
            }
            _ => panic!("Expected Float"),
        }
    }
}
