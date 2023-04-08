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
    RBrace,
    LBracket,
    RBracket,
    Op(Operator),
    Arrow,
    DoubleDot,
    Comma,
    Equals,

    Primitive(Primitive),
}

impl Default for Type {
    fn default() -> Self {
        Type::Nl
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    True,
    False,
    If,
    Else,
    Fun,
    Mut,
    Var,
    Break,
    Loop,
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SourcePos {
    line: u32,
    column: u32,
}

impl SourcePos {
    pub fn new(column: u32, line: u32) -> Self {
        SourcePos { line, column }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub pos: SourcePos,
    pub value: Type,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            pos: SourcePos::new(1, 1),
            value: Default::default(),
        }
    }
}

impl MakeErr for Token {
    fn into_err(self, message: &str) -> CompilerError {
        CompilerError::new(self.pos.line as usize, self.pos.column as usize, 1, message)
    }
}
