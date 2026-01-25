use crate::parser::Expr;

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
}