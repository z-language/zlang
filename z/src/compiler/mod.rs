use std::collections::HashMap;

use zasm::{
    builder::{self, Operand, Reg, Variable},
    func,
};

use crate::parser::{
    ast::{Assign, BinOp, Module, Node, Operator, Primitive, VariableDef},
    ZResult,
};

pub struct Compiler<'guard> {
    module: builder::Module<'guard>,
    builder: builder::Builder,
    vars: HashMap<String, Variable>,
}

macro_rules! make_operand {
    ($x:expr, $self:ident) => {
        match $x {
            Node::Constant(c) => match c.value {
                Primitive::Int(i) => Operand::Int(i),
                _ => todo!("Support."),
            },
            Node::BinOp(binop) => Operand::Reg($self.build_binop(binop)),
            Node::Name(name) => {
                let var = $self.vars.get(&name.id).unwrap().clone();
                Operand::Var(var)
            }
            oops => panic!("This can't be a binop operand: {:?}", oops),
        }
    };
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
            _ => panic!("Unknown node {:?}", node),
        }

        Ok(())
    }

    fn build_assign(&mut self, assign: Assign) {
        let value = make_operand!(*assign.value, self);
        self.builder
            .assign_var(value, self.vars.get(&assign.target).unwrap());
    }

    fn build_binop(&mut self, binop: BinOp) -> Reg {
        let left = make_operand!(*binop.left, self);
        let right = make_operand!(*binop.right, self);

        match binop.op {
            Operator::Add => self.builder.build_add(left, right),
            _ => todo!("Operator not impl."),
        }
    }

    fn build_var(&mut self, var: VariableDef) {
        let value = make_operand!(*var.value, self);
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
}
