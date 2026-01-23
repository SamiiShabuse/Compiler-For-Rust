use crate::tokens::Token;
use crate::tokenizer::Tokenizer;

pub enum Expr{
    This,
    Constant(i64),
    Variable(String),
    BinaryOp{
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    
}