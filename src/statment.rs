use crate::parser::Expr;

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    Assignment { name: String, value: Expr },
    Discard(Expr),
    Return(Expr),

    FieldWrite {
        object: Expr,
        field: String,
        value: Expr,
    },

    IfOnly {
        condition: Expr,
        body: Vec<Statement>,
    },

    If {
        condition: Expr,
        then_body: Vec<Statement>,
        else_body: Vec<Statement>,
    },

    While {
        condition: Expr,
        body: Vec<Statement>,
    },
}