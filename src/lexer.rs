#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: String,
    pub r#type: TokenType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Word,
    String,
    GreaterThan,
    Pipe,
}

pub fn lex(line: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut in_string = ' ';
    let mut token = String::new();

    let mut iter = line.chars().peekable();
    while let Some(c) = iter.next() {
        if in_string != ' ' && c == '\n' {
            return Err("Syntax Error: Unterminated string".to_string());
        }
        if in_string != ' ' && c != in_string {
            token.push(c);
            continue;
        }
        match c {
            '"' | '\'' => {
                if in_string.is_whitespace() {
                    if !token.is_empty() {
                        tokens.push(Token {
                            value: token.clone(),
                            r#type: TokenType::Word,
                        });
                        token.clear();
                    }
                    in_string = c;
                } else {
                    in_string = ' ';
                    tokens.push(Token {
                        value: token.clone(),
                        r#type: TokenType::String,
                    });
                    token.clear();
                }
            }
            '>' => {
                if !token.is_empty() {
                    tokens.push(Token {
                        value: token.clone(),
                        r#type: TokenType::Word,
                    });
                    token.clear();
                }
                tokens.push(Token {
                    value: '>'.to_string(),
                    r#type: TokenType::GreaterThan,
                });
            }
            '|' => {
                if !token.is_empty() {
                    tokens.push(Token {
                        value: token.clone(),
                        r#type: TokenType::Word,
                    });
                    token.clear();
                }
                tokens.push(Token {
                    value: '|'.to_string(),
                    r#type: TokenType::Pipe,
                });
            }
            ' ' => {
                if !token.is_empty() {
                    tokens.push(Token {
                        value: token.clone(),
                        r#type: TokenType::Word,
                    });
                    token.clear();
                }
            }
            _ => token.push(c),
        }
    }

    if in_string != ' ' {
        return Err("Syntax Error: Unterminated string".to_string());
    }

    if !token.is_empty() {
        tokens.push(Token {
            value: token.clone(),
            r#type: TokenType::Word,
        });
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_string() {
        let tokens = lex("echo 'Hello, World!'").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "Hello, World!".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_string_invalid() {
        let tokens = lex("echo 'Hello, World!").unwrap_err();
        assert_eq!(tokens, "Syntax Error: Unterminated string");
    }

    #[test]
    fn test_lexer_string_double() {
        let tokens = lex("echo \"Hello, World!\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "Hello, World!".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_string_double_invalid() {
        let tokens = lex("echo \"Hello, World!").unwrap_err();
        assert_eq!(tokens, "Syntax Error: Unterminated string");
    }

    #[test]
    fn test_lexer_string_mixed() {
        let tokens = lex("echo 'Hello, World!\"").unwrap_err();
        assert_eq!(tokens, "Syntax Error: Unterminated string");
    }

    #[test]
    fn test_lexer_string_mixed_double() {
        let tokens = lex("echo \"Hello, World!'").unwrap_err();
        assert_eq!(tokens, "Syntax Error: Unterminated string");
    }

    #[test]
    fn test_lexer_string_mixed_single_valid() {
        let tokens = lex("echo 'Hello, \"World\"!'").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "Hello, \"World\"!".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_string_mixed_double_valid() {
        let tokens = lex("echo \"Hello, 'World'!\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "Hello, 'World'!".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_string_no_space_before_string() {
        let tokens = lex("echo\"Hello, World!\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "Hello, World!".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_word() {
        let tokens = lex("echo Hello, World!").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "Hello,".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "World!".to_string(),
                    r#type: TokenType::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_word_with_space() {
        let tokens = lex("echo Hello, World! ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "Hello,".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: "World!".to_string(),
                    r#type: TokenType::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_greater_than() {
        let tokens = lex("echo > file.txt").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: ">".to_string(),
                    r#type: TokenType::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    r#type: TokenType::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_greater_than_with_space() {
        let tokens = lex("echo > file.txt ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: ">".to_string(),
                    r#type: TokenType::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    r#type: TokenType::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_greater_than_with_string() {
        let tokens = lex("echo > 'file.txt'").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: ">".to_string(),
                    r#type: TokenType::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_greater_than_with_string_and_space() {
        let tokens = lex("echo > 'file.txt' ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: ">".to_string(),
                    r#type: TokenType::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_greater_than_with_string_double() {
        let tokens = lex("echo > \"file.txt\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: ">".to_string(),
                    r#type: TokenType::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_greater_than_with_string_double_and_space() {
        let tokens = lex("echo > \"file.txt\" ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    r#type: TokenType::Word
                },
                Token {
                    value: ">".to_string(),
                    r#type: TokenType::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    r#type: TokenType::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_greater_than_with_string_mixed() {
        let tokens = lex("echo > 'file.txt\"").unwrap_err();
        assert_eq!(tokens, "Syntax Error: Unterminated string");
    }

    #[test]
    fn test_lexer_greater_than_with_string_and_file() {
        let tokens = lex("\"Hello, World!\" > file.txt").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "Hello, World!".to_string(),
                    r#type: TokenType::String
                },
                Token {
                    value: ">".to_string(),
                    r#type: TokenType::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    r#type: TokenType::Word
                }
            ]
        );
    }
}
