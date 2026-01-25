use crate::parser::Expr;

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    Assignment { name: String, value: Expr },
    Discard(Expr),
    Return(Expr),
}