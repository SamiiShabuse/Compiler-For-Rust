#[derive(Debug, Clone)]
pub enum RValue {
    Immediate(i64),
    Variable(String), // %x
    GlobalVariable(String), // @vtble
}

#[derive(Debug, Clone)]
pub enum LValue {
    Variable(String), // %x
    GlobalVariable(String), // @vtble
    Deref(Box<RValue>), // *%x or *@vtble
}

#[derive(Debug, Clone)]
pub enum Primitive {
    PrintRValue(RValue),
    Assign { dst:  String, src: RValue },
    BinOp { dst: String, lhs: RValue, op: char,  rhs: RValue }, // op is e.g. +, -, *, /
}

#[derive(Debug, Clone)]
pub enum Term {
    Return(RValue),
    Jump(String), // label
    CJump { condition: String, target: String, fallthrough: String }, // condition is variable name
    Fail(String), // error message
}

#[derive(Debug, Clone)]
pub struct Block {
    pub label: String,
    pub prims: Vec<Primitive>,
    pub terms: Term,
}

#[derive(Debug, Clone)]
pub struct ProgramIR {
    pub data: Vec<String>,
    pub blocks: Vec<Block>,
}