use std::collections::HashMap;

use crate::parser::ast::Primitive;

mod core;
#[allow(dead_code, non_camel_case_types)]
mod instructions;

pub struct Compiler {
    variable_map: Vec<String>,

    // name and address of func in const pool and address of func in function_store
    function_map: HashMap<String, (usize, usize)>,
    function_store: Vec<Vec<u8>>,

    constants: Vec<Primitive>,
}
