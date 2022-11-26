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
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: u32,
    pub pos: u32,
    pub value: String,
    pub t_type: Type,
}
