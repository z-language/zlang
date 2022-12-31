use std::iter::Peekable;

use crate::lexer::{token::Type, Lexer};

pub mod ast;
mod core;
// #[cfg(test)]
// mod parser_tests;
mod rpn;

pub struct Parser<'guard> {
    tokens: Peekable<Lexer<'guard>>,
    prev: Type,
}
