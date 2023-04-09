pub mod builder;
pub mod constants;
pub mod func;
pub mod types;

macro_rules! free {
    ($x:ident, $builder:ident) => {
        $builder.free_reg($x)
    };
    ($x:expr, $builder:ident) => {
        let tmp = $x;
        $builder.free_reg(tmp)
    };
}

#[cfg(test)]
mod tests {
    use crate::{builder::*, constants::STDOUT_FILENO, func::Function, types::*};

    #[test]
    fn test_basic() {
        let mut module = Module::new();
        let mut builder = Builder::new();

        let mut main = Function::new("main");

        let mut adder = Function::new("adder");
        let result = builder.build_add(Operand::Int(2), Operand::Int(3));
        free!(
            builder.build_add(Operand::Reg(result), Operand::Int(4)),
            builder
        );
        builder.write_to_fn(&mut adder);

        builder.build_call(&adder);
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
