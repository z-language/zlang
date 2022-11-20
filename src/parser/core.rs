use crate::tokenizer::Token;

use super::ast::Module;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Module {
        let module = Module::new();

        module
    }
}
