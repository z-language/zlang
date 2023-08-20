use crate::{
    error::{CompilerError, MakeErr},
    parser::ast::Primitive,
};
use zasm::types::Operator;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Type {
    Keyword(Keyword),
    Word(String),
    LParen,
    RParen,
    #[default]
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
    Not,

    Primitive(Primitive),
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
    pub line: u32,
    pub column: u32,
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
        Token {
            pos: SourcePos::new(1, 1),
            value: Type::default(),
        }
    }
}

impl MakeErr for Token {
    fn into_err(self, message: &str) -> CompilerError {
        CompilerError::new(self.pos.line as usize, self.pos.column as usize, 1, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pos;

    #[test]
    fn test_default() {
        let t = Token::default();
        let expected = Token {
            pos: pos!(1, 1),
            value: Type::Nl,
        };

        assert_eq!(t, expected);
    }

    #[test]
    fn test_make_err() {
        let t = Token {
            pos: pos!(2, 5),
            value: Type::Arrow,
        };
        let message = "Test error.";
        let test_case = t.into_err(message);
        let expected = CompilerError::new(5, 2, 1, message);

        assert_eq!(test_case, expected)
    }
}
