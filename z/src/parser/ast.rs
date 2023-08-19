use zasm::types;

use crate::lexer::token::SourcePos;

#[derive(Debug, PartialEq)]
pub struct Module {
    pub body: Vec<Node>,
}

impl Module {
    pub fn new() -> Self {
        Module { body: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub args: Vec<Node>,
    pub body: Vec<Node>,
    pub returns: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub test: Box<Node>,
    pub run: Scope,
    pub orelse: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Scope {
    pub body: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Loop {
    pub body: Scope,
}

#[derive(Debug, PartialEq)]
pub struct Arg {
    pub name: String,
    pub annotation: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub target: String,
    pub value: Box<Node>,
    pub pos: SourcePos,
}

#[derive(Debug, PartialEq)]
pub struct VariableDef {
    pub name: String,
    pub mutable: bool,
    pub value: Box<Node>,
}
#[derive(Debug, PartialEq)]
pub struct Constant {
    pub value: Primitive,
}

#[derive(Debug, PartialEq)]
pub struct BinOp {
    pub left: Box<Node>,
    pub op: types::Operator,
    pub right: Box<Node>,
}
#[derive(Debug, PartialEq)]
pub struct Name {
    pub id: String,
}

#[derive(Debug, PartialEq)]
pub struct List {
    pub elements: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub func: Name,
    pub args: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    FunctionDef(FunctionDef),
    VariableDef(VariableDef),
    Assign(Assign),
    Arg(Arg),
    Constant(Constant),
    BinOp(BinOp),
    Name(Name, SourcePos),
    Call(Call),
    If(If),
    Scope(Scope),
    Loop(Loop),
    Return(Return),
    List(List),

    Break(SourcePos),
    None,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Primitive {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),

    #[default]
    None,
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::Int(i) => i.to_string(),
            Primitive::Float(f) => f.to_string(),
            Primitive::Str(s) => s.clone(),
            Primitive::Bool(b) => b.to_string(),
            Primitive::None => "".to_owned(),
        }
    }
}
