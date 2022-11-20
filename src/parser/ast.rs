#[derive(Debug)]
pub struct Module {
    body: Vec<Node>,
}

impl Module {
    pub fn new() -> Self {
        Module { body: vec![] }
    }
}

#[derive(Debug)]
struct FunctionDef {
    name: String,
    args: Vec<Node>,
    body: Vec<Node>,
    returns: Box<Node>,
}
#[derive(Debug)]
struct Assign {
    target: Box<Node>,
    value: Box<Node>,
}
#[derive(Debug)]
struct Constant {
    value: Primitive,
}
#[derive(Debug)]
struct BinOp {
    left: Box<Node>,
    op: Operator,
    right: Box<Node>,
}
#[derive(Debug)]
struct Name {
    id: String,
}

#[derive(Debug)]
struct Expr {
    value: Box<Node>,
}
#[derive(Debug)]
struct Call {
    func: Name,
    args: Vec<Node>,
}
#[derive(Debug)]
enum Node {
    FunctionDef(FunctionDef),
    Assign(Assign),
    Constant(Constant),
    BinOp(BinOp),
    Name(Name),
    Expr(Expr),
    Call(Call),
}

#[derive(Debug)]
enum Primitive {
    Int,
    Float,
    Str,
    Bool,
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}
