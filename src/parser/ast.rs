#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub body: Vec<Node>,
}

impl Module {
    pub fn new() -> Self {
        Module { body: vec![] }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub args: Vec<Node>,
    pub body: Vec<Node>,
    pub returns: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub test: Box<Node>,
    pub run: Box<Node>,
    pub orelse: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Loop {
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arg {
    pub name: String,
    pub annotation: Box<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    pub target: String,
    pub value: Box<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct VariableDef {
    pub name: String,
    pub mutable: bool,
    pub value: Box<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Constant {
    pub value: Primitive,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BinOp {
    pub left: Box<Node>,
    pub op: Operator,
    pub right: Box<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Name {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub func: Name,
    pub args: Vec<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    FunctionDef(FunctionDef),
    VariableDef(VariableDef),
    Assign(Assign),
    Arg(Arg),
    Constant(Constant),
    BinOp(BinOp),
    Name(Name),
    Call(Call),
    If(If),
    Scope(Scope),
    Loop(Loop),
    Return(Return),

    Break,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Int(i32),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    DoubleEquals,
    Mod,
}
