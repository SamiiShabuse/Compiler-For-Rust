use crate::token::Token;
use crate::token::TokenType;

pub struct Tokenizer {
    tex: Vec<char>,
    current: usize,
    cached_token: Option<Token>,
}   

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        // ask professor if he prefers to use Self { ... } or Tokenizer { ... }
        Tokenizer {
            tex: input.chars().collect(),
            current: 0,
            cached_token: None,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.cached_token.is_none() {
            self.cached_token = self.next_token();
        }
        self.cached_token.as_ref()
    }

        pub fn next_token(&mut self) -> Option<Token> {
        if let Some(token) = self.cached_token.take() {
            return Some(token);
        }

        // Tokenization logic goes here
        None
    }

}