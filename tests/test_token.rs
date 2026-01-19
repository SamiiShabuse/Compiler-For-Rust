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