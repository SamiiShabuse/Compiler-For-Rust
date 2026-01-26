use crate::ir::*;
use crate::statement::Statement;
use crate::parser::Expr;

pub fn lower_main(stmts: &[Statement]) -> ProgramIR {
    let mut prims = Vec::new();

    for s in stmts {
        match s {
            Statement::Print(e) => {
                if let Expr::Constant(n) = e {
                    prims.push(Prim::PrintRValue(RValue::Immediate(*n)));
                } else {
                    panic!("For now, only print(constant) is supported in lowering");
                }
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
