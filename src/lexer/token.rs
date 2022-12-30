use crate::{
    error::{CompilerError, MakeErr},
    parser::ast::{Operator, Primitive},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Keyword(Keyword),
    Word(String),
    LParen,
    RParen,
    Nl,
    LBrace,
    Rbrace,
    Op(Operator),
    Arrow,
    DoubleDot,
    Comma,
    Equals,

    Primitive(Primitive),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    True,
    False,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SourcePos {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub pos: SourcePos,
    pub value: Type,
}

impl MakeErr for Token {
    fn into_err(self, message: &str) -> CompilerError {
        CompilerError::new(self.pos.line as usize, self.pos.column as usize, 1, message)
    }
}
