use self::ast::Node;
use crate::tokenizer::token::Token;

#[allow(dead_code)] // for now
pub mod ast;
mod core;
#[cfg(test)]
mod parser_tests;

pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    body: Vec<Node>,
}
