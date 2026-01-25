use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::statment::Statement;

#[derive(Debug)]
pub enum Expr{
    This,
    Constant(i64),
    Variable(String),
    BinaryOp{
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
    MethodCall{
        object: Box<Expr>,
        method: String,
        arguments: Vec<Expr>,
    },
    FieldRead{
        object: Box<Expr>,
        field: String,
    },
    ClassReference(String),
}

pub struct Parser{
    tokenizer: Tokenizer,
}

impl Parser{
    pub fn new(tokenizer: Tokenizer) -> Self{
        Self { tokenizer }
    }

    pub fn parse_statement(&mut self) -> Statement {
        match self.tokenizer.next_token() {
            Token::Print => {
                match self.tokenizer.next_token() {
                    Token::LeftParen => {}
                    other => panic!("Expected '(' after 'print' but found {:?}", other),
                }
                let expr = self.parse_expression();
                match self.tokenizer.next_token() {
                    Token::RightParen => {}
                    other => panic!("Expected ')' after expression but found {:?}", other),
                }
                Statement::Print(expr)
            }
            other => {
                panic!("Unexpected token while parsing statement: {:?}", other) 
                }
            }
        }

    pub fn parse_expression(&mut self) -> Expr {
        match self.tokenizer.next_token() {
            Token::Eof => panic!("No expressions to parse: EOF"),
            Token::Number(number) => Expr::Constant(number),
            Token::Identifier(identifier) => Expr::Variable(identifier),

            Token::LeftParen => {
                let lhs = self.parse_expression();
                let op = match self.tokenizer.next_token() {
                    Token::Operator(op) => op,
                    other => panic!("Expected operator after left parenthesis but found {:?}", other),
                };
                let rhs = self.parse_expression();
                match self.tokenizer.next_token() {
                    Token::RightParen => {}
                    other => panic!("Expected right parenthesis but found {:?}", other),
                }
                Expr::BinaryOp { left: Box::new(lhs), op, right: Box::new(rhs) }
            }

            Token::Ampersand => {
                let base = self.parse_expression();
                match self.tokenizer.next_token() {
                    Token::Dot => {}
                    other => panic!("Expected '.' after '&' but found {:?}", other),
                }
                let field = match self.tokenizer.next_token() {
                    Token::Identifier(field) => field,
                    other => panic!("Expected field name after '&.' but found {:?}", other),
                };
                Expr::FieldRead { object: Box::new(base), field }
            }

            Token::Caret => {
                let base =self.parse_expression();
                match self.tokenizer.next_token() {
                    Token::Dot => {}
                    other => panic!("Expected '.' after '^' but found {:?}", other),
                }
                let methodname = match self.tokenizer.next_token() {
                    Token::Identifier(string) => string,
                    other => panic!("Expected method name after '^.' but found {:?}", other),
                };
                match self.tokenizer.next_token() {
                    Token::LeftParen => {}
                    other => panic!("Expected '(' after method name but found {:?}", other),
                }

                let mut args = Vec::new();
                while self.tokenizer.peek() != Token::RightParen {
                    let e = self.parse_expression();
                    eprintln!("Parsed argument expression: {:?}", e);
                    args.push(e);

                    // Now either comma or right parenthesis
                    if self.tokenizer.peek() == Token::Comma {
                        let _ = self.tokenizer.next_token(); // consume comma
                    }
                }

                // now consume ')'
                match self.tokenizer.next_token() {
                    Token::RightParen => {}
                    other => panic!("Expected ')' after method arguments but found {:?}", other),
                }

                Expr::MethodCall {
                    object: Box::new(base),
                    method: methodname,
                    arguments: args,
                }
            }

            Token::AtSign => {
                let class_name = match self.tokenizer.next_token() {
                    Token::Identifier(string) => string,
                    other => panic!("Expected class name after '@' but found {:?}", other),
                };
                Expr::ClassReference(class_name)
            }

            Token::This => Expr::This,
            

            other => panic!("Unexpected token while parsing expression: {:?}", other),
        }
            
    }
}