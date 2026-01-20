use compiler_for_rust::token::Token;

#[test]
fn test_type_fixed_punctuations() {
    let tokens = vec![
        Token::LeftParen,
        Token::RightParen,
        Token::LeftBrace,
        Token::RightBrace,
        Token::Caret,
        Token::Ampersand,
        Token::AtSign,
        Token::Not,
        Token::Dot,
        Token::Colon,
        Token::Comma,

    ];

    for token in tokens {
        match token {
            Token::LeftParen
            | Token::RightParen
            | Token::LeftBrace
            | Token::RightBrace
            | Token::Caret
            | Token::Ampersand
            | Token::AtSign
            | Token::Not
            | Token::Dot
            | Token::Colon
            | Token::Comma => {}
            _ => panic!("Token is not of fixed punctuation type"),
        }
    }
}

#[test]
fn test_type_keywords() {
    let tokens = vec![
        Token::This,
        Token::If,
        Token::IfOnly,
        Token::While,
        Token::Return,
        Token::Print,
        Token::Eof,
    ];

    for token in tokens {
        match token {
            Token::This
            | Token::If
            | Token::IfOnly
            | Token::While
            | Token::Return
            | Token::Print
            | Token::Eof => {}
            _ => panic!("Token is not of keyword type"),
        }
    }
}

#[test]
fn test_type_tokens_with_data() {
    let tokens = vec![
        Token::Operator('+'),
        Token::Number(42),
        Token::Identifier("variable".to_string()),
    ];

    for token in tokens {
        match token {
            Token::Operator(_) | Token::Number(_) | Token::Identifier(_) => {}
            _ => panic!("Token is not of data type"),
        }
    }
}

#[test]
fn test_token_type() {
    let token_types = vec![
        (Token::LeftParen, "LeftParen"),
        (Token::RightParen, "RightParen"),
        (Token::LeftBrace, "LeftBrace"),
        (Token::RightBrace, "RightBrace"),
        (Token::Caret, "Caret"),
        (Token::Ampersand, "Ampersand"),
        (Token::AtSign, "AtSign"),
        (Token::Not, "Not"),
        (Token::Dot, "Dot"),
        (Token::Colon, "Colon"),
        (Token::Comma, "Comma"),
        (Token::This, "This"),
        (Token::If, "If"),
        (Token::IfOnly, "IfOnly"),
        (Token::While, "While"),
        (Token::Return, "Return"),
        (Token::Print, "Print"),
        (Token::Eof, "Eof"),
        (Token::Operator('+'), "Operator"),
        (Token::Number(42), "Number"),
        (Token::Identifier("variable".to_string()), "Identifier"),
    ];

    for (token, expected_type) in token_types {
        assert_eq!(token.token_type(), expected_type);
    }
}