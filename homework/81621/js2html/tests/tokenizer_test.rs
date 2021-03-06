#[cfg(test)]
mod tests {
    use js2html::lexer::token_type::TokenType;
    use js2html::lexer::tokenizer::{Token, Tokenizer};

    #[test]
    fn empty_test() {
        let mut lexer = Tokenizer::new("function main() {}".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Function,
                    number: None,
                    string: Some("function".to_string())
                },
                Token {
                    r#type: TokenType::Identifier,
                    number: None,
                    string: Some("main".to_string())
                },
                Token {
                    r#type: TokenType::LParenthesis,
                    number: None,
                    string: Some("(".to_string())
                },
                Token {
                    r#type: TokenType::RParenthesis,
                    number: None,
                    string: Some(")".to_string())
                },
                Token {
                    r#type: TokenType::LCurlyBrace,
                    number: None,
                    string: Some("{".to_string())
                },
                Token {
                    r#type: TokenType::RCurlyBrace,
                    number: None,
                    string: Some("}".to_string())
                }
            ]
        );
    }

    #[test]
    fn simple_test() {
        let mut lexer = Tokenizer::new(
            "
function main() {
    var a = 123;
    console.log(a);
}"
            .to_string(),
        );

        lexer.tokenize_whitespace = true;
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some("\n".to_string())
                },
                Token {
                    r#type: TokenType::Function,
                    number: None,
                    string: Some("function".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some(" ".to_string())
                },
                Token {
                    r#type: TokenType::Identifier,
                    number: None,
                    string: Some("main".to_string())
                },
                Token {
                    r#type: TokenType::LParenthesis,
                    number: None,
                    string: Some("(".to_string())
                },
                Token {
                    r#type: TokenType::RParenthesis,
                    number: None,
                    string: Some(")".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some(" ".to_string())
                },
                Token {
                    r#type: TokenType::LCurlyBrace,
                    number: None,
                    string: Some("{".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some("\n    ".to_string())
                },
                Token {
                    r#type: TokenType::Var,
                    number: None,
                    string: Some("var".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some(" ".to_string())
                },
                Token {
                    r#type: TokenType::Identifier,
                    number: None,
                    string: Some("a".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some(" ".to_string())
                },
                Token {
                    r#type: TokenType::Assign,
                    number: None,
                    string: Some("=".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some(" ".to_string())
                },
                Token {
                    r#type: TokenType::Number,
                    number: Some(123f64),
                    string: None
                },
                Token {
                    r#type: TokenType::Semicolon,
                    number: None,
                    string: Some(";".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some("\n    ".to_string())
                },
                Token {
                    r#type: TokenType::Identifier,
                    number: None,
                    string: Some("console".to_string())
                },
                Token {
                    r#type: TokenType::Dot,
                    number: None,
                    string: Some(".".to_string())
                },
                Token {
                    r#type: TokenType::Identifier,
                    number: None,
                    string: Some("log".to_string())
                },
                Token {
                    r#type: TokenType::LParenthesis,
                    number: None,
                    string: Some("(".to_string())
                },
                Token {
                    r#type: TokenType::Identifier,
                    number: None,
                    string: Some("a".to_string())
                },
                Token {
                    r#type: TokenType::RParenthesis,
                    number: None,
                    string: Some(")".to_string())
                },
                Token {
                    r#type: TokenType::Semicolon,
                    number: None,
                    string: Some(";".to_string())
                },
                Token {
                    r#type: TokenType::Whitespace,
                    number: None,
                    string: Some("\n".to_string())
                },
                Token {
                    r#type: TokenType::RCurlyBrace,
                    number: None,
                    string: Some("}".to_string())
                }
            ]
        );
    }

    #[test]
    fn test_square_braces() {
        let mut lexer = Tokenizer::new("[1]".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::LSquareBrace,
                    string: Some("[".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(1f64)
                },
                Token {
                    r#type: TokenType::RSquareBrace,
                    string: Some("]".to_string()),
                    number: None
                }
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Tokenizer::new("123".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![Token {
                r#type: TokenType::Number,
                string: None,
                number: Some(123f64)
            }]
        );

        lexer = Tokenizer::new("00123.0".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![Token {
                r#type: TokenType::Number,
                string: None,
                number: Some(123f64)
            }]
        );

        lexer = Tokenizer::new("00.3".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![Token {
                r#type: TokenType::Number,
                string: None,
                number: Some(0.3f64)
            }]
        );
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Tokenizer::new("niceVar".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![Token {
                r#type: TokenType::Identifier,
                string: Some("niceVar".to_string()),
                number: None
            }]
        );

        lexer = Tokenizer::new("nice".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![Token {
                r#type: TokenType::Identifier,
                string: Some("nice".to_string()),
                number: None
            }]
        );

        lexer = Tokenizer::new("nice_var".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![Token {
                r#type: TokenType::Identifier,
                string: Some("nice_var".to_string()),
                number: None
            }]
        );
    }

    #[test]
    fn test_operators() {
        let mut lexer = Tokenizer::new("a=3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Assign,
                    string: Some("=".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("4+3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(4f64)
                },
                Token {
                    r#type: TokenType::Plus,
                    string: Some("+".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("4-3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(4f64)
                },
                Token {
                    r#type: TokenType::Minus,
                    string: Some("-".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("4*3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(4f64)
                },
                Token {
                    r#type: TokenType::Multiply,
                    string: Some("*".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("4/3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(4f64)
                },
                Token {
                    r#type: TokenType::Divide,
                    string: Some("/".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("a<3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Less,
                    string: Some("<".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("a<=3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::LessOrEqual,
                    string: Some("<=".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("a != 3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::NotEquals,
                    string: Some("!=".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("a==3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Equals,
                    string: Some("==".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("a>3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::More,
                    string: Some(">".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("a>=3".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::MoreOrEqual,
                    string: Some(">=".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                }
            ]
        );

        lexer = Tokenizer::new("&a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::BitwiseAnd,
                    string: Some("&".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
            ]
        );

        lexer = Tokenizer::new("a.to".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Dot,
                    string: Some(".".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("to".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("test=>a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("test".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Arrow,
                    string: Some("=>".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("test&a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("test".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::BitwiseAnd,
                    string: Some("&".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("test&&a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("test".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::And,
                    string: Some("&&".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("test|a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("test".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::BitwiseOr,
                    string: Some("|".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("test||a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("test".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Or,
                    string: Some("||".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("test^a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("test".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::BitwiseXor,
                    string: Some("^".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("test^^a".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("test".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Xor,
                    string: Some("^^".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                }
            ]
        );

        lexer = Tokenizer::new("b=='a'".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("b".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Equals,
                    string: Some("==".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::CharString,
                    string: Some("'a'".to_string()),
                    number: None
                },
            ]
        );

        lexer = Tokenizer::new("b=='a\\''".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("b".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Equals,
                    string: Some("==".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::CharString,
                    string: Some("'a''".to_string()),
                    number: None
                },
            ]
        );

        lexer = Tokenizer::new("b==\"a\\\"\"".to_string());
        lexer.populate_tokens();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("b".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Equals,
                    string: Some("==".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::String,
                    string: Some("\"a\"\"".to_string()),
                    number: None
                },
            ]
        );
    }

    #[test]
    fn test_comments() {
        let mut lexer = Tokenizer::new("//Hey there".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![Token {
                r#type: TokenType::Comment,
                string: Some("//Hey there".to_string()),
                number: None
            },]
        );

        lexer = Tokenizer::new("a=3; //Hey there".to_string());
        lexer.populate_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    r#type: TokenType::Identifier,
                    string: Some("a".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Assign,
                    string: Some("=".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Number,
                    string: None,
                    number: Some(3f64)
                },
                Token {
                    r#type: TokenType::Semicolon,
                    string: Some(";".to_string()),
                    number: None
                },
                Token {
                    r#type: TokenType::Comment,
                    string: Some("//Hey there".to_string()),
                    number: None
                },
            ]
        );
    }
}
