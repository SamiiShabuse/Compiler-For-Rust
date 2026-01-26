#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Fixed Punctuations
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Caret,
    Ampersand,
    AtSign,
    Not,
    Dot,
    Colon,
    Comma,
    Equal,

    // Keywords
    This,
    Else,
    If,
    IfOnly,
    While,
    Return,
    Print,
    Eof,
    // Tokens with data
    Operator(char),
    Number(i64), // idk if this needs to be float or int yet
    Identifier(String),
}

impl Token {
    pub fn token_type(&self) -> &'static str {
        match self {
            Token::LeftParen => "LeftParen",
            Token::RightParen => "RightParen",
            Token::LeftBrace => "LeftBrace",
            Token::RightBrace => "RightBrace",
            Token::Caret => "Caret",
            Token::Ampersand => "Ampersand",
            Token::AtSign => "AtSign",
            Token::Not => "Not",
            Token::Dot => "Dot",
            Token::Colon => "Colon",
            Token::Comma => "Comma",
            Token::This => "This",
            Token::If => "If",
            Token::IfOnly => "IfOnly",
            Token::Else => "Else",
            Token::While => "While",
            Token::Return => "Return",
            Token::Print => "Print",
            Token::Eof => "Eof",
            Token::Operator(_) => "Operator",
            Token::Number(_) => "Number",
            Token::Identifier(_) => "Identifier",
            Token::Equal => "Equal",
        }
    }
}