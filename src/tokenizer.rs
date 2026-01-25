use crate::token::Token;

pub struct Tokenizer {
    text: Vec<char>,
    current: usize,
    cached_token: Option<Token>,
}   

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        // ask professor if he prefers to use Self { ... } or Tokenizer { ... }
        Tokenizer {
            text: input.chars().collect(),
            current: 0,
            cached_token: None,
        }
    }

    pub fn peek(&mut self) -> Token {
        if self.cached_token.is_none() {
            self.cached_token = Some(self.next_token());
        }
        self.cached_token.as_ref().unwrap().clone()
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.cached_token.take() {
            token
        } else {
            self.advance_token()
        }
    }

    fn advance_token(&mut self) -> Token {
        // skip the whitespace
        while self.current < self.text.len() && self.text[self.current].is_whitespace() {
            self.current += 1;
        }
        // return Eof if we reach the end of the input
        if self.current >= self.text.len() {
            return Token::Eof;
        }

        let ch = self.text[self.current];
        match ch {
            '(' => {
                self.current += 1;
                Token::LeftParen
            }
            ')' => {
                self.current += 1;
                Token::RightParen
            }
            '{' => {
                self.current += 1;
                Token::LeftBrace
            }
            '}' => {
                self.current += 1;
                Token::RightBrace
            }
            '^' => {
                self.current += 1;
                Token::Caret
            }
            '&' => {
                self.current += 1;
                Token::Ampersand
            }
            '@' => {
                self.current += 1;
                Token::AtSign
            }
            '!' => {
                self.current += 1;
                Token::Not
            }
            '.' => {
                self.current += 1;
                Token::Dot
            }
            ':' => {
                self.current += 1;
                Token::Colon
            }
            ',' => {
                self.current += 1;
                Token::Comma
            }

            '=' => {
                self.current += 1;
                Token::Equal
            }

            '+' | '-' | '*' | '/' | '%' => {
                self.current += 1;
                Token::Operator(ch)
            }

            _ => {
                if ch.is_ascii_digit() {
                    let start = self.current;
                    self.current += 1;
                    while self.current < self.text.len() && self.text[self.current].is_ascii_digit() {
                        self.current += 1;
                    }
                    let s: String = self.text[start..self.current].iter().collect();

                    let value = s.parse::<i64>().unwrap();
                    Token::Number(value)
                } else if ch.is_ascii_alphabetic() {
                    let start = self.current;
                    self.current += 1;
                    while self.current < self.text.len() && self.text[self.current].is_ascii_alphanumeric() {
                        self.current += 1;
                    }
                    let frag: String = self.text[start..self.current].iter().collect();

                    match frag.as_str() {
                        "this" => Token::This,
                        "if" => Token::If,
                        "ifonly" => Token::IfOnly,
                        "while" => Token::While,
                        "return" => Token::Return,
                        "print" => Token::Print,
                        _ => Token::Identifier(frag),
                    }
                } else {
                    panic!("Unexpected character: {}", ch); // if any appear here add it to the token 
                }
            }
        }
    }

}