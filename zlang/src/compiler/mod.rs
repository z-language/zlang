use zasm::builder;

use crate::parser::{ast::Module, ZResult};

pub struct Compiler;

impl Compiler {
    pub fn compile(&mut self, source: Module) -> ZResult<()> {
        let mut module = builder::Module::new();
        let mut builder = builder::Builder::new();

        Ok(())
    }
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {}
    }
}
