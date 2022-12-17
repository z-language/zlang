use crate::parser::ast::Primitive;

mod core;
#[allow(dead_code, non_camel_case_types)]
mod instructions;

pub struct Compiler {
    variable_map: Vec<String>,
    constants: Vec<Primitive>,
}
