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

struct InternalVar {
    inner: Variable,
    mutable: bool,
    scope: u32,
}

impl InternalVar {
    pub fn new(inner: Variable, mutable: bool, scope: u32) -> Self {
        Self {
            inner,
            mutable,
            scope,
        }
    }
}

pub struct Compiler<'guard> {
    module: Module<'guard>,
    builder: Builder,
    vars: HashMap<String, InternalVar>,

    scope_depth: u32,
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
                let tmp = self.build_binop(binop)?;
                self.builder.free_reg(tmp);
            }
            Node::VariableDef(var) => self.build_var(var)?,
            Node::Assign(ass) => self.build_assign(ass)?,
            Node::Call(call) => self.build_call(call)?,
            Node::Return(ret) => self.build_return(ret)?,
            Node::If(case) => self.build_if(case)?,
            Node::Loop(r#loop) => self.build_loop(r#loop)?,
            Node::Break => self.build_break(),
            _ => panic!("Unknown node {:?}", node),
        }

        Ok(())
    }

    fn build_if(&mut self, case: If) -> ZResult<()> {
        self.add_scope();
        // TODO: orelse
        self.handle_node(*case.test)?;
        self.builder.write_raw("    cmp eax, 1\n");

        let label1 = self.builder.get_label();

        self.builder.build_jump(&label1, Jump::NotEqual);
        for node in case.run.body {
            self.handle_node(node)?;
        }

        self.builder.insert_label(&label1);

        self.clear_scope();
        Ok(())
    }

    fn add_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn clear_scope(&mut self) {
        self.vars
            .retain(|_, inner_var| inner_var.scope < self.scope_depth);
        self.scope_depth -= 1;
    }

    fn build_loop(&mut self, r#loop: Loop) -> ZResult<()> {
        self.add_scope();
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

        self.clear_scope();
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

    fn build_call(&mut self, mut call: Call) -> ZResult<()> {
        if call.func.id == ASM {
            return Ok(self.build_inline_asm(call));
        }

        let n_args = call.args.len();
        call.args.reverse();
        for arg in call.args {
            let value = self.make_operand(arg)?;
            self.builder.build_push(value);
        }
        self.builder.call_by_name(&call.func.id);

        // Temporarely create the rsp register that gets deleted
        // after this function exits.
        let reg = Operand::Reg(Reg::new("rsp"));
        self.builder
            .build_op(reg, Operand::Int((n_args * 8) as i32), Operator::Add);
        Ok(())
    }

    fn build_return(&mut self, ret: Return) -> ZResult<()> {
        let operand = self.make_operand(*ret.value)?;
        self.builder.build_return(operand);
        Ok(())
    }

    fn build_assign(&mut self, assign: Assign) -> ZResult<()> {
        let value = self.make_operand(*assign.value)?;

        let var = match self.vars.get(&assign.target) {
            Some(var) => var,
            None => {
                return Err(CompilerError::new(
                    1,
                    1,
                    1,
                    &*format!("Variable '{}', not found in scope.", assign.target),
                ))
            }
        };
        // checks if var is mutable
        if !var.mutable {
            return Err(CompilerError::new(1, 2, 1, "Variable is imutable."));
        }

        self.builder.assign_var(value, &var.inner);
        Ok(())
    }

    fn build_binop(&mut self, binop: BinOp) -> ZResult<Reg> {
        let left = self.make_operand(*binop.left)?;
        let right = self.make_operand(*binop.right)?;

        Ok(self.builder.build_op(left, right, binop.op))
    }

    fn build_var(&mut self, var: VariableDef) -> ZResult<()> {
        let value = self.make_operand(*var.value)?;
        let inner = self.builder.make_var(value);
        self.vars.insert(
            var.name,
            InternalVar::new(inner, var.mutable, self.scope_depth),
        );

        Ok(())
    }
}

impl<'guard> Compiler<'guard> {
    pub fn new() -> Self {
        Compiler {
            module: Module::new(),
            builder: Builder::new(),
            vars: HashMap::default(),
            current_labels: vec![],
            scope_depth: 0,
        }
    }

    pub fn make_operand(&mut self, node: Node) -> ZResult<Operand> {
        match node {
            Node::Constant(c) => match c.value {
                Primitive::Int(i) => Ok(Operand::Int(i)),
                Primitive::Str(str) => {
                    let ptr = self.module.add_string(&str);
                    Ok(Operand::StrPtr(ptr))
                }
                _ => todo!("Support."),
            },
            Node::BinOp(binop) => Ok(Operand::Reg(self.build_binop(binop)?)),
            Node::Name(name) => {
                let var = match self.vars.get(&name.id) {
                    Some(var) => var.inner.clone(),
                    None => {
                        return Err(CompilerError::new(
                            1,
                            1,
                            1,
                            &*format!("Variable '{}' not found in scope.", name.id),
                        ))
                    }
                };
                Ok(Operand::Var(var))
            }
            Node::Call(call) => {
                self.build_call(call)?;
                Ok(Operand::Reg(Reg::new("eax")))
            }
            oops => panic!("This can't be an operand: {:?}", oops),
        }
    }
}
