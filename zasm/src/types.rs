#[derive(Debug)]
pub struct StrPtr(usize);

impl Source for StrPtr {}
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

pub trait Store {}
pub trait Source {}

impl Source for u8 {}
impl Source for u16 {}
impl Source for u32 {}
impl Source for u64 {}
impl Source for u128 {}
impl Source for usize {}
impl Source for i8 {}
impl Source for i16 {}
impl Source for i32 {}
impl Source for i64 {}
impl Source for i128 {}
impl Source for isize {}
