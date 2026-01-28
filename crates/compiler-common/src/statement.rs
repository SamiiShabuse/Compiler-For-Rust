#[derive(Debug, Clone)]
pub enum Expr {
    This,
    Constant(i64),
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        arguments: Vec<Expr>,
    },
    FieldRead {
        object: Box<Expr>,
        field: String,
    },
    ClassReference(String),
}

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
