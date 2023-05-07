use builder::Reg;
use func::Function;

pub mod builder;
pub mod constants;
pub mod func;
pub mod types;

pub struct Builder {
    buffer: String,
    registers: Vec<Reg>,
    offset: u32,
    reserved: u32,
}

pub struct Module<'guard> {
    globals: Vec<&'guard str>,
    strings: Vec<String>,
    functions: Vec<Function>,
}
#[cfg(test)]
mod tests {
    use crate::{builder::*, func::Function, types::*, *};

    #[test]
    fn test_basic() {
        let mut module = Module::new();
        let mut builder = Builder::new();

        let mut main = Function::new("main");

        let mut adder = Function::new("adder");
        let result = builder.build_op(Operand::Int(2), Operand::Int(3), Operator::Add);
        let result = builder.build_op(Operand::Reg(result), Operand::Int(4), Operator::Add);
        builder.free_reg(result);

        builder.write_to_fn(&mut adder);

        // builder.build_call(&adder);
        builder.write_to_fn(&mut main);

        module.add_func(adder);
        module.add_func(main);

        module.write_to_file("out.asm").unwrap();
    }

    #[test]
    fn test_vars() {
        let mut module = Module::new();
        let mut builder = Builder::new();

        let mut main = Function::new("main");

        let _i = builder.make_var(Operand::Int(3));

        builder.write_to_fn(&mut main);

        module.add_func(main);
    }
}
