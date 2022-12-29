use crate::error::{CompilerError, MakeErr};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Keyword(String),
    Word(String),
    Lparen,
    Rparen,
    Nl,
    Lbrack,
    Rbrack,
    Op(String),
    Arrow,
    DoubleDot,
    Comma,
    Equals,

    String(String),
    Int(String),
    Float(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourcePos {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub pos: SourcePos,
    pub value: Type,
}

impl MakeErr for Token {
    fn into_err(&self, message: &str) -> CompilerError {
        CompilerError::new(
            self.pos.line as usize,
            self.pos.column as usize,
            // TODO: self.value.len(),
            1,
            message,
        )
    }

    fn into_err_offset(&self, offset: i32, message: &str) -> CompilerError {
        CompilerError::new(
            self.pos.line as usize,
            ((self.pos.column as i32) + offset) as usize,
            // TODO: self.value.len(),
            1,
            message,
        )
    }
}
