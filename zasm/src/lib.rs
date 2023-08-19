use builder::Reg;
use func::Function;

pub mod builder;
pub mod constants;
pub mod func;
pub mod types;

pub struct Builder {
    buffer: String,
    registers: Vec<Reg>,
    offset: i32,
    reserved: u32,
    label_count: u32,
}

pub struct Module<'guard> {
    globals: Vec<&'guard str>,
    strings: Vec<String>,
    functions: Vec<Function>,
}
