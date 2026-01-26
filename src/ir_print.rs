use crate::ir::*;

pub fn fmt_rvalue(v: &RValue) -> String {
    match v {
        RValue::Immediate(i) => i.to_string(),
        RValue::Variable(s) => s.clone(),
        RValue::GlobalVariable(s) => s.clone(),
    }
}

pub fn fmt_prim(p: &Prim) -> String {
    match p {
        Prim::PrintRValue(v) => format!("print {}", fmt_rvalue(v)),
        Prim::Assign { dst, src } => format!("{} = {}", dst, fmt_rvalue(src)),      
    }
}

pub fn fmt_term(t: &Term) -> String {
    match t {
        Term::Return(v) => format!("return {}", fmt_rvalue(v)),
        Term::Jump(label) => format!("jump {}", label),
        Term::CJump { condition, target, fallthrough } => {
            format!("cjump {} {} {}", condition, target, fallthrough)
        }
        Term::Fail(msg) => format!("fail {}", msg),
    }
}

pub fn print_program(p: &ProgramIR) -> String{
    let mut out = String::new();
    out.push_str("Data:\n");
    for d in &p.data {
        out.push_str(&format!("  {}\n", d));
    }
    out.push_str("Code:\n");

    for b in &p.blocks {
        out.push_str(&format!("{}:\n", b.label));
        for prim in &b.prims {
            out.push_str(&format!("  {}\n", fmt_prim(prim)));
        }
        out.push_str(&format!("  {}\n", fmt_term(&b.terms)));
    }

    out
}