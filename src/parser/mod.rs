use std::iter::Peekable;

use crate::lexer::Lexer;

pub mod ast;
mod core;
// #[cfg(test)]
// mod parser_tests;
mod rpn;

pub struct Parser<'guard> {
    tokens: Peekable<Lexer<'guard>>,

    scope: usize,
    building_binop: Vec<bool>,
}
