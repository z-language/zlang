use std::collections::HashMap;

use crate::parser::ast::Primitive;

mod core;
#[allow(non_camel_case_types, dead_code)]
mod instructions;

pub struct Compiler {
    scope: usize,
    variable_map: Vec<Vec<String>>,

    // first usize is the current pos of loop, the second is the total size of the loop
    loop_store: Vec<(usize, usize)>,
    pos: usize,

    // name and address of func stored in const pool and address of func in function_store
    function_map: HashMap<String, (usize, usize)>,
    function_store: Vec<Vec<u8>>,

    constants: Vec<Primitive>,

    iteration: i32,
}
