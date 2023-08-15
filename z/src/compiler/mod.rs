use std::collections::HashMap;

use zasm::{
    builder::{Operand, Reg, Variable},
    func,
    types::{Jump, Label, Operator},
    Builder, Module,
};

use crate::{
    error::CompilerError,
    grammar::ASM,
    parser::{
        ast::{Assign, BinOp, Call, If, Loop, Module as Mod, Node, Primitive, Return, VariableDef},
        ZResult,
    },
};

pub struct Compiler<'guard> {
    module: Module<'guard>,
    builder: Builder,
    vars: HashMap<String, (Variable, bool)>,

    current_labels: Vec<Label>,
}

impl<'guard> Compiler<'guard> {
    pub fn compile(&mut self, source: Mod) -> ZResult<()> {
        self.module = Module::new();
        self.builder = Builder::new();

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
            Node::Assign(ass) => self.build_assign(ass)?,
            Node::Call(call) => self.build_call(call),
            Node::Return(ret) => self.build_return(ret),
            Node::If(case) => self.build_if(case)?,
            Node::Loop(r#loop) => self.build_loop(r#loop)?,
            Node::Break => self.build_break(),
            _ => panic!("Unknown node {:?}", node),
        }

        Ok(())
    }

    fn build_if(&mut self, case: If) -> ZResult<()> {
        // TODO: orelse
        self.handle_node(*case.test)?;
        self.builder.write_raw("    cmp eax, 1\n");

        let label1 = self.builder.get_label();

        self.builder.build_jump(&label1, Jump::NotEqual);
        for node in case.run.body {
            self.handle_node(node)?;
        }

        self.builder.insert_label(&label1);

        Ok(())
    }

    fn build_loop(&mut self, r#loop: Loop) -> ZResult<()> {
        let label_start = self.builder.get_label();
        let label_end = self.builder.get_label();
        self.current_labels.push(label_end);

        self.builder.insert_label(&label_start);
        for node in r#loop.body.body {
            self.handle_node(node)?;
        }
        self.builder.build_jump(&label_start, Jump::Always);

        self.builder.insert_label(
            &self
                .current_labels
                .pop()
                .expect("Label was pushed in same function."),
        );
        Ok(())
    }

    fn build_break(&mut self) {
        let label = self.current_labels.last().unwrap();
        self.builder.build_jump(label, Jump::Always);
    }

    fn build_inline_asm(&mut self, call: Call) {
        for arg in call.args {
            if let Node::Constant(constant) = arg {
                let text = constant.value.to_string();
                self.builder.write_raw_fmt(&text);
            } else {
                panic!("Only constants can be used in inline asm.")
            }
        }
    }

    fn build_call(&mut self, mut call: Call) {
        if call.func.id == ASM {
            return self.build_inline_asm(call);
        }

        let n_args = call.args.len();
        call.args.reverse();
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

    fn build_return(&mut self, ret: Return) {
        let operand = self.make_operand(*ret.value);
        self.builder.build_return(operand);
    }

    fn build_assign(&mut self, assign: Assign) -> ZResult<()> {
        let value = self.make_operand(*assign.value);

        let var = self.vars.get(&assign.target).unwrap();
        // checks if var is mutable
        if !var.1 {
            return Err(CompilerError::new(1, 2, 1, "Variable is imutable."));
        }

        self.builder.assign_var(value, &var.0);
        Ok(())
    }

    fn build_binop(&mut self, binop: BinOp) -> Reg {
        let left = self.make_operand(*binop.left);
        let right = self.make_operand(*binop.right);

        self.builder.build_op(left, right, binop.op)
    }

    fn build_var(&mut self, var: VariableDef) {
        let value = self.make_operand(*var.value);
        let i = self.builder.make_var(value);
        self.vars.insert(var.name, (i, var.mutable));
    }
}

impl<'guard> Compiler<'guard> {
    pub fn new() -> Self {
        Compiler {
            module: Module::new(),
            builder: Builder::new(),
            vars: HashMap::default(),
            current_labels: vec![],
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
                let var = self.vars.get(&name.id).unwrap().0.clone();
                Operand::Var(var)
            }
            Node::Call(call) => {
                self.build_call(call);
                Operand::Reg(Reg::new("eax"))
            }
            oops => panic!("This can't be an operand: {:?}", oops),
        }
    }
}
