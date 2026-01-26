use core::panic;
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

    pub fn expect(&mut self, want: Token) {
        let got = self.tokenizer.next_token();
        if got != want {
            panic!("Expected token {:?} but got {:?}", want, got);
        }
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        // used to sexpect { stmtements... }
        self.expect(Token::LeftBrace);
        
        let mut body = Vec::new();

        // keep parsing statements until hittting }
        while self.tokenizer.peek() != Token::RightBrace {
            let statment = self.parse_statement();
            body.push(statment);
        }

        self.expect(Token::RightBrace);
        body
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

            Token::Identifier(name) => {
                match self.tokenizer.next_token() {
                    Token::Equal => {}
                    other => panic!("Expected '=' after identifier but found {:?}", other),
                }
                let rhs = self.parse_expression();
                if name == "_" {
                    return Statement::Discard(rhs);
                } else {
                    Statement::Assignment { name, value: rhs }
                }
            }

            Token::Return => {
                let expr = self.parse_expression();
                Statement::Return(expr)
            }

            Token::Eof => panic!("Unexpected EOF"),

            Token::Not =>  {
                let object = self.parse_expression();
                
                match self.tokenizer.next_token() {
                    Token::Dot => {}
                    other => panic!("Expected '.' after 'not' but found {:?}", other),
                }

                let field = match self.tokenizer.next_token() {
                    Token::Identifier(field) => field,
                    other => panic!("Expected field name after 'not.' but found {:?}", other),
                };

                match self.tokenizer.next_token() {
                    Token::Equal => {}
                    other => panic!("Expected '=' after '!e.f' but found {:?}", other),
                }

                let value = self.parse_expression();

                Statement::FieldWrite { object, field, value }
            }

            Token::IfOnly => {
                let condition = self.parse_expression();

                match self.tokenizer.next_token() {
                    Token::Colon => {}
                    other => panic!("Expected ':' after 'ifonly' condition but found {:?}", other),
                }

                let body = self.parse_block();

                Statement::IfOnly { condition, body }
            }

            Token::If => {
                let condition = self.parse_expression();

                match self.tokenizer.next_token() {
                    Token::Colon => {}
                    other => panic!("Expected ':' after 'if' condition but found {:?}", other),
                }

                let then_body = self.parse_block();

                match self.tokenizer.next_token() {
                    Token::Else => {}
                    other => panic!("Expected 'else' after 'if' condition but found {:?}", other),
                }

                let else_body = self.parse_block();

                Statement::If { condition, then_body, else_body  }
            }

            Token::While => {
                let condition = self.parse_expression();

                match self.tokenizer.next_token() {
                    Token::Colon => {}
                    other => panic!("Expected ':' after 'while' condition but found {:?}", other),
                }

                let body = self.parse_block();

                Statement::While { condition, body }
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