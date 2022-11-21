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
pub struct Arg {
    name: String,
    annotation: Box<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    pub target: Box<Node>,
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
pub struct Expr {
    pub value: Box<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub func: Name,
    pub args: Vec<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    FunctionDef(FunctionDef),
    Arg(Arg),
    Assign(Assign),
    Constant(Constant),
    BinOp(BinOp),
    Name(Name),
    Expr(Expr),
    Call(Call),

    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Int(isize),
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
}
