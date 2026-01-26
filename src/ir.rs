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
pub enum Prim {
    PrintRValue(RValue),
    Assign { dst:  String, src: RValue },
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
    pub prims: Vec<Prim>,
    pub terms: Term,
}

#[derive(Debug, Clone)]
pub struct ProgramIR {
    pub data: Vec<String>,
    pub blocks: Vec<Block>,
}