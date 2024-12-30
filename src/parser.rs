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

    pub fn parse(&mut self) -> Command {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Command {
        if self.tokens.peek().is_none() {
            return Command::NONE;
        }
        let mut command = self.parse_expression();

        while let Some(token) = self.tokens.peek() {
            command = match token.r#type {
                TokenType::GreaterThan | TokenType::Pipe => self.parse_binary(command),
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Cd);
                assert_eq!(args, Vec::<Token>::new());
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Cd);
                assert_eq!(
                    args,
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Cd);
                assert_eq!(
                    args,
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Exit);
                assert_eq!(args, Vec::<Token>::new());
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Exit);
                assert_eq!(
                    args,
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Exit);
                assert_eq!(
                    args,
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Ls);
                assert_eq!(args, Vec::<Token>::new());
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Ls);
                assert_eq!(
                    args,
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Builtin { builtin, args } => {
                assert_eq!(builtin, Builtin::Ls);
                assert_eq!(
                    args,
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::String(value) => {
                assert_eq!(value, "hello");
            }
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_parse_external() {
        let tokens = vec![Token {
            value: "helloworld".to_string(),
            r#type: TokenType::Word,
        }];

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::External { name, args } => {
                assert_eq!(
                    name,
                    Token {
                        value: "helloworld".to_string(),
                        r#type: TokenType::Word,
                    }
                );
                assert_eq!(args, Vec::<Token>::new());
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::External { name, args } => {
                assert_eq!(
                    name,
                    Token {
                        value: "helloworld".to_string(),
                        r#type: TokenType::Word,
                    }
                );
                assert_eq!(
                    args,
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
        let command = Parser::new(tokens.iter().peekable()).parse();
        assert_eq!(command, Command::NONE);
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Redirect {
                source,
                destination,
            } => {
                assert_eq!(
                    *source,
                    Command {
                        command: CommandType::Builtin {
                            builtin: Builtin::Ls,
                            args: Vec::new()
                        },
                        stdin: None,
                    }
                );
                assert_eq!(
                    *destination,
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

        let command = Parser::new(tokens.iter().peekable()).parse();
        match command.command {
            CommandType::Redirect {
                source,
                destination,
            } => {
                assert_eq!(
                    *source,
                    Command {
                        command: CommandType::String("ls".to_string()),
                        stdin: None,
                    }
                );
                assert_eq!(
                    *destination,
                    Command {
                        command: CommandType::String("output.txt".to_string()),
                        stdin: None,
                    }
                );
            }
            _ => panic!("Expected Redirect"),
        }
    }
}
