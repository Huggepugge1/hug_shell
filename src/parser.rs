use crate::command::builtins;
use crate::command::{Command, CommandType};
use crate::lexer::{Token, TokenType};

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
                match t.r#type {
                    TokenType::SemiColon => {
                        self.tokens.next();
                        if self.tokens.peek().is_none() {
                            break;
                        }
                    }
                    _ => {
                        return vec![Command {
                            command: CommandType::Error(format!(
                                "Expected `;`, found `{}`",
                                t.value
                            )),
                            stdin: None,
                        }]
                    }
                }
            }
            let command = self.parse_statement();
            match command.command {
                CommandType::Error(_) => {
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
            command = match token.r#type {
                TokenType::GreaterThan | TokenType::Pipe => self.parse_binary(command),
                TokenType::SemiColon => {
                    break;
                }
                _ => Command {
                    command: CommandType::Error("Unexpected token".to_string()),
                    stdin: None,
                },
            };
        }
        command
    }

    fn parse_expression(&mut self) -> Command {
        if let Some(token) = self.tokens.peek() {
            match token.r#type {
                TokenType::Word => self.parse_word(),
                TokenType::String => self.parse_string(),
                TokenType::Boolean => self.parse_boolean(),
                TokenType::SemiColon => Command {
                    command: CommandType::None,
                    stdin: None,
                },
                _ => Command {
                    command: CommandType::Error("Unexpected token".to_string()),
                    stdin: None,
                },
            }
        } else {
            Command {
                command: CommandType::Error("Unexpected end of input".to_string()),
                stdin: None,
            }
        }
    }

    fn parse_word(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        let builtin = if builtins::is_builtin(&token.value) {
            Some(builtins::get(&token.value))
        } else {
            None
        };
        let mut args = Vec::new();

        while let Some(token) = self.tokens.peek() {
            match token.r#type {
                TokenType::Word => args.push(self.tokens.next().unwrap().clone()),
                TokenType::String => args.push(self.tokens.next().unwrap().clone()),
                _ => break,
            }
        }

        if let Some(builtin) = builtin {
            Command {
                command: CommandType::Builtin { builtin, args },
                stdin: None,
            }
        } else {
            Command {
                command: CommandType::External {
                    name: token.clone(),
                    args,
                },
                stdin: None,
            }
        }
    }

    fn parse_string(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        Command {
            command: CommandType::String(token.value.clone()),
            stdin: None,
        }
    }

    fn parse_boolean(&mut self) -> Command {
        let token = self.tokens.next().unwrap();
        Command {
            command: CommandType::Boolean(token.value.parse().unwrap()),
            stdin: None,
        }
    }

    fn parse_binary(&mut self, command: Command) -> Command {
        match self.tokens.next() {
            Some(token) => match token.r#type {
                TokenType::GreaterThan => Command {
                    command: CommandType::Redirect {
                        source: Box::new(command),
                        destination: Box::new(self.parse_expression()),
                    },
                    stdin: None,
                },
                TokenType::Pipe => Command {
                    command: CommandType::Pipe {
                        source: Box::new(command),
                        destination: Box::new(self.parse_expression()),
                    },
                    stdin: None,
                },
                _ => Command {
                    command: CommandType::Error("Unexpected token".to_string()),
                    stdin: None,
                },
            },
            None => Command {
                command: CommandType::Error("Unexpected end of input".to_string()),
                stdin: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Builtin;

    #[test]
    fn test_parse_cd() {
        let tokens = vec![Token {
            value: "cd".to_string(),
            r#type: TokenType::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_cd_with_one_arg() {
        let tokens = vec![
            Token {
                value: "cd".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "/home".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(
                    *args,
                    vec![Token {
                        value: "/home".to_string(),
                        r#type: TokenType::Word,
                    }]
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
                r#type: TokenType::Word,
            },
            Token {
                value: "/home".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "arg2".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(
                    *args,
                    vec![
                        Token {
                            value: "/home".to_string(),
                            r#type: TokenType::Word,
                        },
                        Token {
                            value: "arg2".to_string(),
                            r#type: TokenType::Word,
                        }
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
            r#type: TokenType::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Exit);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_exit_with_one_arg() {
        let tokens = vec![
            Token {
                value: "exit".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "1".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Exit);
                assert_eq!(
                    *args,
                    vec![Token {
                        value: "1".to_string(),
                        r#type: TokenType::Word,
                    }]
                );
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_exit_with_multiple_args() {
        let tokens = vec![
            Token {
                value: "exit".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "1".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "2".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Exit);
                assert_eq!(
                    *args,
                    vec![
                        Token {
                            value: "1".to_string(),
                            r#type: TokenType::Word,
                        },
                        Token {
                            value: "2".to_string(),
                            r#type: TokenType::Word,
                        }
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
            r#type: TokenType::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_ls_with_one_arg() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "arg1".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(
                    *args,
                    vec![Token {
                        value: "arg1".to_string(),
                        r#type: TokenType::Word
                    }]
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
                r#type: TokenType::Word,
            },
            Token {
                value: "arg1".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "arg2".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(
                    *args,
                    vec![
                        Token {
                            value: "arg1".to_string(),
                            r#type: TokenType::Word
                        },
                        Token {
                            value: "arg2".to_string(),
                            r#type: TokenType::Word
                        }
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
            r#type: TokenType::String,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::String(ref value) => {
                assert_eq!(value, "hello");
            }
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_parse_boolean() {
        let tokens = vec![Token {
            value: "true".to_string(),
            r#type: TokenType::Boolean,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Boolean(value) => {
                assert_eq!(value, true);
            }
            _ => panic!("Expected Boolean"),
        }

        let tokens = vec![Token {
            value: "false".to_string(),
            r#type: TokenType::Boolean,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Boolean(value) => {
                assert_eq!(value, false);
            }
            _ => panic!("Expected Boolean"),
        }
    }

    #[test]
    fn test_parse_external() {
        let tokens = vec![Token {
            value: "helloworld".to_string(),
            r#type: TokenType::Word,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::External { ref name, ref args } => {
                assert_eq!(
                    *name,
                    Token {
                        value: "helloworld".to_string(),
                        r#type: TokenType::Word,
                    }
                );
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected External"),
        }

        let tokens = vec![
            Token {
                value: "helloworld".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "arg1".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: "arg2".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::External { ref name, ref args } => {
                assert_eq!(
                    *name,
                    Token {
                        value: "helloworld".to_string(),
                        r#type: TokenType::Word,
                    }
                );
                assert_eq!(
                    *args,
                    vec![
                        Token {
                            value: "arg1".to_string(),
                            r#type: TokenType::Word,
                        },
                        Token {
                            value: "arg2".to_string(),
                            r#type: TokenType::Word,
                        }
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
                r#type: TokenType::Word,
            },
            Token {
                value: ">".to_string(),
                r#type: TokenType::GreaterThan,
            },
            Token {
                value: "output.txt".to_string(),
                r#type: TokenType::String,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Redirect {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        command: CommandType::Builtin {
                            builtin: Builtin::Ls,
                            args: Vec::new()
                        },
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        command: CommandType::String("output.txt".to_string()),
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
                r#type: TokenType::String,
            },
            Token {
                value: ">".to_string(),
                r#type: TokenType::GreaterThan,
            },
            Token {
                value: "output.txt".to_string(),
                r#type: TokenType::String,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Redirect {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        command: CommandType::String("ls".to_string()),
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        command: CommandType::String("output.txt".to_string()),
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
                r#type: TokenType::Word,
            },
            Token {
                value: "|".to_string(),
                r#type: TokenType::Pipe,
            },
            Token {
                value: "grep".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Pipe {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        command: CommandType::Builtin {
                            builtin: Builtin::Ls,
                            args: Vec::new()
                        },
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        command: CommandType::External {
                            name: Token {
                                value: "grep".to_string(),
                                r#type: TokenType::Word
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
                r#type: TokenType::String,
            },
            Token {
                value: "|".to_string(),
                r#type: TokenType::Pipe,
            },
            Token {
                value: "grep".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        match commands[0].command {
            CommandType::Pipe {
                ref source,
                ref destination,
            } => {
                assert_eq!(
                    **source,
                    Command {
                        command: CommandType::String("ls".to_string()),
                        stdin: None,
                    }
                );
                assert_eq!(
                    **destination,
                    Command {
                        command: CommandType::External {
                            name: Token {
                                value: "grep".to_string(),
                                r#type: TokenType::Word
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
                r#type: TokenType::Word,
            },
            Token {
                value: ";".to_string(),
                r#type: TokenType::SemiColon,
            },
            Token {
                value: "cd".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 2);

        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }

        match commands[1].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn test_parse_three_commands() {
        let tokens = vec![
            Token {
                value: "ls".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: ";".to_string(),
                r#type: TokenType::SemiColon,
            },
            Token {
                value: "cd".to_string(),
                r#type: TokenType::Word,
            },
            Token {
                value: ";".to_string(),
                r#type: TokenType::SemiColon,
            },
            Token {
                value: "pwd".to_string(),
                r#type: TokenType::Word,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 3);

        match commands[0].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Ls);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }

        match commands[1].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Cd);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }

        match commands[2].command {
            CommandType::Builtin {
                ref builtin,
                ref args,
            } => {
                assert_eq!(*builtin, Builtin::Pwd);
                assert_eq!(*args, Vec::<Token>::new());
            }
            _ => panic!("Expected BuiltIn"),
        }
    }

    #[test]
    fn parse_only_semicolon() {
        let tokens = vec![Token {
            value: ";".to_string(),
            r#type: TokenType::SemiColon,
        }];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 1);
        match commands[0].command {
            CommandType::None => (),
            _ => panic!("Expected None"),
        }
    }

    #[test]
    fn parse_multiple_semicolons() {
        let tokens = vec![
            Token {
                value: ";".to_string(),
                r#type: TokenType::SemiColon,
            },
            Token {
                value: ";".to_string(),
                r#type: TokenType::SemiColon,
            },
        ];

        let commands = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(commands.len(), 2);
        match commands[0].command {
            CommandType::None => (),
            _ => panic!("Expected None"),
        }
        match commands[1].command {
            CommandType::None => (),
            _ => panic!("Expected None"),
        }
    }
}
