use std::fmt::Display;

use crate::error::{CompilerError, MakeErr};

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
    Equals,

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

impl MakeErr for Token {
    fn into_err(&self, message: &str) -> CompilerError {
        CompilerError::new(
            self.line as usize,
            self.pos as usize,
            self.value.len(),
            message,
        )
    }

    fn into_err_offset(&self, offset: i32, message: &str) -> CompilerError {
        CompilerError::new(
            self.line as usize,
            ((self.pos as i32) + offset) as usize,
            self.value.len(),
            message,
        )
    }
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
