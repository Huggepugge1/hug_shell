use crate::command::builtins;
use crate::command::Command;
use crate::lexer::{Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> Command {
    if tokens.is_empty() {
        return Command::None;
    }

    match tokens[0].r#type {
        TokenType::Word => {
            if builtins::is_builtin(&tokens[0].value) {
                let builtin = builtins::get(&tokens[0].value);
                let args = match tokens.len() > 1 {
                    true => tokens[1..].into_iter().map(|t| t.clone()).collect(),
                    false => Vec::new(),
                };
                Command::Builtin { builtin, args }
            } else {
                let args = match tokens.len() > 1 {
                    true => tokens[1..].into_iter().map(|t| t.clone()).collect(),
                    false => Vec::new(),
                };
                let name = tokens[0].clone();
                Command::External { name, args }
            }
        }
        TokenType::String => {
            if tokens.len() > 1 {
                Command::Error("Too many arguments".to_string())
            } else {
                Command::String(tokens[0].value.clone())
            }
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::Builtin { builtin, args } => {
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

        let command = parse(tokens);
        match command {
            Command::String(value) => {
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

        let command = parse(tokens);
        match command {
            Command::External { name, args } => {
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

        let command = parse(tokens);
        match command {
            Command::External { name, args } => {
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
        let command = parse(tokens);
        assert_eq!(command, Command::None);
    }
}
