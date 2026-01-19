pub enum Token {
    // Fixed Punctuations
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    CARET,
    AMPERSAND,
    ATSIGN,
    NOT,
    DOT,
    COLON,
    COMMA,

    // Keywords
    THIS,
    IF,
    IFONLY,
    WHILE,
    RETURN,
    PRINT,
    EOF,

    // Tokens with data
    OPERATOR(char),
    NUMBER(i64), // idk if this needs to be float or int yet
    IDENTIFIER(String),
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        match self {
            Token::LEFT_PAREN => TokenType::LEFT_PAREN,
            Token::RIGHT_PAREN => TokenType::RIGHT_PAREN,
            Token::LEFT_BRACE => TokenType::LEFT_BRACE,
            Token::RIGHT_BRACE => TokenType::RIGHT_BRACE,
            Token::CARET => TokenType::CARET,
            Token::AMPERSAND => TokenType::AMPERSAND,
            Token::ATSIGN => TokenType::ATSIGN,
            Token::NOT => TokenType::NOT,
            Token::DOT => TokenType::DOT,
            Token::COLON => TokenType::COLON,
            Token::COMMA => TokenType::COMMA,
            Token::THIS => TokenType::THIS,
            Token::IF => TokenType::IF,
            Token::IFONLY => TokenType::IFONLY,
            Token::WHILE => TokenType::WHILE,
            Token::RETURN => TokenType::RETURN,
            Token::PRINT => TokenType::PRINT,
            Token::EOF => TokenType::EOF,
            Token::OPERATOR(_) => TokenType::OPERATOR,
            Token::NUMBER(_) => TokenType::NUMBER,
            Token::IDENTIFIER(_) => TokenType::IDENTIFIER,
        }
    }
}