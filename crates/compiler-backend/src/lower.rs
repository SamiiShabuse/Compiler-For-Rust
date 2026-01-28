use compiler_ir::*;
use compiler_common::{statement::Statement, statement::Expr};

pub fn lower_main(stmts: &[Statement]) -> ProgramIR {
    let mut prims = Vec::new();
    let mut temps = TempGen::new();

    for s in stmts {
        match s {
            Statement::Print(e) => {
                let v = lower_expression(e, &mut prims, &mut temps);
                prims.push(Primitive::PrintRValue(v));
            }
            
            Statement::Assignment { name, value } => {
                let v = lower_expression(value, &mut prims, &mut temps);
                prims.push(Primitive::Assign { dst: var_name(name), src: v });
            } 

            Statement::Return(expr) => {
                let v = lower_expression(expr, &mut prims, &mut temps);
                return ProgramIR {
                    data: vec![],
                    blocks: vec![Block {
                        label: "main".to_string(),
                        prims,
                        terms: Term::Return(v),
                    }],
                };
            }

            // TODO: for now allow discard only if it's a pure expression we can lower.
            // fix later because of side effects
            Statement::Discard(e) => {
                let _ = lower_expression(e, &mut prims, &mut temps);
            }

            _ => panic!("For now, only print(...) statements are supported in lowering"),
        }
    }

    ProgramIR {
        data: vec![],
        blocks: vec![Block {
            label: "main".to_string(),
            prims,
            terms: Term::Return(RValue::Immediate(0)),
        }],
    }
}

struct TempGen {
    n: usize,
}

impl TempGen {
    fn new() -> Self {
        TempGen { n: 0 }
    }

    fn next(&mut self) -> String {
        let name = format!("%t{}", self.n);
        self.n += 1;
        name
    }
}

pub fn var_name(name: &str) -> String {
    format!("%{}", name)
}

fn lower_expression(expr: &Expr, primitives: &mut Vec<Primitive>, temp_gen: &mut TempGen) -> RValue {
    match expr {
        Expr::Constant(n) => RValue::Immediate(*n),
        Expr::Variable(name) => RValue::Variable(var_name(name)),
        Expr::BinaryOp {left: lhs, op, right: rhs} => {
            let lhs_rvalue = lower_expression(lhs, primitives, temp_gen);
            let rhs_rvalue = lower_expression(rhs, primitives, temp_gen);
            let dst = temp_gen.next();
            primitives.push(Primitive::BinOp {
                dst: dst.clone(),
                lhs: lhs_rvalue,
                op: *op,
                rhs: rhs_rvalue,
            });
            RValue::Variable(dst)
        }
        _ => panic!("lower_expr: not supported yet: {:?}", expr),
    }
}
