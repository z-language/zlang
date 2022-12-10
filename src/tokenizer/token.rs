use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Keyword,
    Word,

    Lparen,
    Rparen,

    Nl,

    Lbrack,
    Rbrack,

    Op,
    Arrow,
    DoubleDot,
    Comma,

    String,
    Int,
    Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: u32,
    pub pos: u32,
    pub value: String,
    pub t_type: Type,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{value: \"{}\", type: \"{:?}\"}}",
            self.value, self.t_type
        )
    }
}
