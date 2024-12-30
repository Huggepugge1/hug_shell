#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(value: &str) -> Self {
        match value {
            ">" => Token {
                value: value.to_string(),
                kind: TokenKind::GreaterThan,
            },
            "|" => Token {
                value: value.to_string(),
                kind: TokenKind::Pipe,
            },
            ";" => Token {
                value: value.to_string(),
                kind: TokenKind::SemiColon,
            },
            "true" | "false" => Token {
                value: value.to_string(),
                kind: TokenKind::Boolean,
            },
            _ => {
                if value.starts_with('\'') && value.ends_with('\'') {
                    Token {
                        value: value[1..value.len() - 1].to_string(),
                        kind: TokenKind::String,
                    }
                } else if value.starts_with('"') && value.ends_with('"') {
                    Token {
                        value: value[1..value.len() - 1].to_string(),
                        kind: TokenKind::String,
                    }
                } else {
                    Token {
                        value: value.to_string(),
                        kind: TokenKind::Word,
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Word,
    String,
    Boolean,

    GreaterThan,
    Pipe,

    SemiColon,
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
                        tokens.push(Token::new(&token));
                        token.clear();
                    }
                    in_string = c;
                } else {
                    in_string = ' ';
                    tokens.push(Token {
                        value: token.clone(),
                        kind: TokenKind::String,
                    });
                    token.clear();
                }
            }
            '>' | '|' | ';' => {
                if !token.is_empty() {
                    tokens.push(Token::new(&token));
                    token.clear();
                }
                tokens.push(Token::new(&c.to_string()));
            }
            ' ' => {
                if !token.is_empty() {
                    tokens.push(Token::new(&token));
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
        tokens.push(Token::new(&token));
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
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello, World!".to_string(),
                    kind: TokenKind::String
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
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello, World!".to_string(),
                    kind: TokenKind::String
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
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello, \"World\"!".to_string(),
                    kind: TokenKind::String
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
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello, 'World'!".to_string(),
                    kind: TokenKind::String
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
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello, World!".to_string(),
                    kind: TokenKind::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_boolean() {
        let tokens = lex("true false").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "true".to_string(),
                    kind: TokenKind::Boolean
                },
                Token {
                    value: "false".to_string(),
                    kind: TokenKind::Boolean
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
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello,".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "World!".to_string(),
                    kind: TokenKind::Word
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
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello,".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "World!".to_string(),
                    kind: TokenKind::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_redirection() {
        let tokens = lex("echo > file.txt").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ">".to_string(),
                    kind: TokenKind::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    kind: TokenKind::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_redirection_with_space() {
        let tokens = lex("echo > file.txt ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ">".to_string(),
                    kind: TokenKind::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    kind: TokenKind::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_redirection_with_string() {
        let tokens = lex("echo > 'file.txt'").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ">".to_string(),
                    kind: TokenKind::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    kind: TokenKind::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_redirection_with_string_and_space() {
        let tokens = lex("echo > 'file.txt' ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ">".to_string(),
                    kind: TokenKind::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    kind: TokenKind::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_redirection_with_string_double() {
        let tokens = lex("echo > \"file.txt\"").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ">".to_string(),
                    kind: TokenKind::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    kind: TokenKind::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_redirection_with_string_double_and_space() {
        let tokens = lex("echo > \"file.txt\" ").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ">".to_string(),
                    kind: TokenKind::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    kind: TokenKind::String
                }
            ]
        );
    }

    #[test]
    fn test_lexer_redirection_with_string_mixed() {
        let tokens = lex("echo > 'file.txt\"").unwrap_err();
        assert_eq!(tokens, "Syntax Error: Unterminated string");
    }

    #[test]
    fn test_lexer_redirection_with_string_and_file() {
        let tokens = lex("\"Hello, World!\" > file.txt").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "Hello, World!".to_string(),
                    kind: TokenKind::String
                },
                Token {
                    value: ">".to_string(),
                    kind: TokenKind::GreaterThan
                },
                Token {
                    value: "file.txt".to_string(),
                    kind: TokenKind::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_pipe() {
        let tokens = lex("echo Hello, World! | wc -l").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello,".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "World!".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "|".to_string(),
                    kind: TokenKind::Pipe
                },
                Token {
                    value: "wc".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "-l".to_string(),
                    kind: TokenKind::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_semi_colon() {
        let tokens = lex("echo Hello, World! ; wc -l").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello,".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "World!".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ";".to_string(),
                    kind: TokenKind::SemiColon
                },
                Token {
                    value: "wc".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "-l".to_string(),
                    kind: TokenKind::Word
                }
            ]
        );
    }

    #[test]
    fn test_lexer_double_semi_colon() {
        let tokens = lex("echo Hello, World! ;; wc -l").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token {
                    value: "echo".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "Hello,".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "World!".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: ";".to_string(),
                    kind: TokenKind::SemiColon
                },
                Token {
                    value: ";".to_string(),
                    kind: TokenKind::SemiColon
                },
                Token {
                    value: "wc".to_string(),
                    kind: TokenKind::Word
                },
                Token {
                    value: "-l".to_string(),
                    kind: TokenKind::Word
                }
            ]
        );
    }
}
