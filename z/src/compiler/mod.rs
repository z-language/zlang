use std::collections::HashMap;

use zasm::{
    builder::{self, Operand, Reg, Variable},
    func,
    types::Operator,
};

use crate::parser::{
    ast::{Assign, BinOp, Call, Module, Node, Primitive, VariableDef},
    ZResult,
};

pub struct Compiler<'guard> {
    module: builder::Module<'guard>,
    builder: builder::Builder,
    vars: HashMap<String, Variable>,
}

impl<'guard> Compiler<'guard> {
    pub fn compile(&mut self, source: Module) -> ZResult<()> {
        self.module = builder::Module::new();
        self.builder = builder::Builder::new();

        for node in source.body {
            self.handle_node(node)?;
        }

        self.module.write_to_file("out.asm").unwrap();

        Ok(())
    }

    fn handle_node(&mut self, node: Node) -> ZResult<()> {
        match node {
            Node::FunctionDef(fun) => {
                let mut f = func::Function::new(&fun.name);

                for node in fun.body {
                    self.handle_node(node)?;
                }

                self.builder.write_to_fn(&mut f);
                self.module.add_func(f);
            }
            Node::BinOp(binop) => {
                let tmp = self.build_binop(binop);
                self.builder.free_reg(tmp);
            }
            Node::VariableDef(var) => self.build_var(var),
            Node::Assign(ass) => self.build_assign(ass),
            Node::Call(call) => self.build_call(call),
            _ => panic!("Unknown node {:?}", node),
        }

        Ok(())
    }

    fn build_call(&mut self, call: Call) {
        let n_args = call.args.len();
        for arg in call.args {
            let value = self.make_operand(arg);
            self.builder.build_push(value);
        }
        self.builder.call_by_name(&call.func.id);

        // Temporarely create the rsp register that gets deleted
        // after this function exits.
        let reg = Operand::Reg(Reg::new("rsp"));
        self.builder
            .build_op(reg, Operand::Int((n_args * 8) as i32), Operator::Add);
    }

    fn build_assign(&mut self, assign: Assign) {
        let value = self.make_operand(*assign.value);
        self.builder
            .assign_var(value, self.vars.get(&assign.target).unwrap());
    }

    fn build_binop(&mut self, binop: BinOp) -> Reg {
        let left = self.make_operand(*binop.left);
        let right = self.make_operand(*binop.right);

        self.builder.build_op(left, right, binop.op)
    }

    fn build_var(&mut self, var: VariableDef) {
        let value = self.make_operand(*var.value);
        let i = self.builder.make_var(value);
        self.vars.insert(var.name, i);
    }
}

impl<'guard> Compiler<'guard> {
    pub fn new() -> Self {
        Compiler {
            module: builder::Module::new(),
            builder: builder::Builder::new(),
            vars: HashMap::default(),
        }
    }

    pub fn make_operand(&mut self, node: Node) -> Operand {
        match node {
            Node::Constant(c) => match c.value {
                Primitive::Int(i) => Operand::Int(i),
                Primitive::Str(str) => {
                    let ptr = self.module.add_string(&str);
                    Operand::StrPtr(ptr)
                }
                _ => todo!("Support."),
            },
            Node::BinOp(binop) => Operand::Reg(self.build_binop(binop)),
            Node::Name(name) => {
                let var = self.vars.get(&name.id).unwrap().clone();
                Operand::Var(var)
            }
            oops => panic!("This can't be an operand: {:?}", oops),
        }
    }
}
