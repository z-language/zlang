#[derive(Debug)]
pub struct StrPtr(usize);

impl ToString for StrPtr {
    fn to_string(&self) -> String {
        format!("str_{}", self.0.to_string())
    }
}
impl StrPtr {
    pub fn new(val: usize) -> Self {
        StrPtr(val)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    DoubleEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    Mod,
}
