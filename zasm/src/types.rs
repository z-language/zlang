#[derive(Debug)]
pub struct StrPtr(usize);

impl ToString for StrPtr {
    fn to_string(&self) -> String {
        format!("str_{}", self.0)
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
    NotEquals,
    Less,
    LessEquals,
    Mod,
}

pub struct Label(String);

impl Label {
    pub fn new(number: u32) -> Self {
        Self(format!(".L{}", number))
    }
}

impl ToString for Label {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub enum Jump {
    NotEqual,
    Always,
}

impl ToString for Jump {
    fn to_string(&self) -> String {
        match self {
            Jump::NotEqual => "jne".to_owned(),
            Jump::Always => "jmp".to_owned(),
        }
    }
}
