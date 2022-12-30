use std::{iter::Peekable, slice::Iter};

use self::ast::Node;
use crate::tokenizer::token::Token;

pub mod ast;
mod core;
#[cfg(test)]
mod parser_tests;
mod rpn;

pub struct Parser {
    body: Vec<Node>,
    tokens: Vec<Token>,
    index: usize,
    building_binop: Vec<bool>,
    scope: usize,
}
