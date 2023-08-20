use std::iter::Peekable;

use crate::{
    error::CompilerError,
    lexer::{token::Token, Lexer},
};

pub mod ast;
mod core;
mod rpn;

pub type ZResult<T> = Result<T, CompilerError>;

pub struct Parser<'guard> {
    tokens: Peekable<Lexer<'guard>>,
    prev: Token,
}
