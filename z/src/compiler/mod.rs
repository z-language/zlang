use std::collections::HashMap;

use regex::Regex;
use zasm::{
    builder::{Operand, Reg, Variable},
    func,
    types::{Jump, Label, Operator},
    Builder, Module,
};

use crate::{
    error::CompilerError,
    grammar,
    lexer::token::SourcePos,
    parser::{
        ast::{
            Assign, BinOp, Call, If, Loop, Module as Mod, Node, Primitive, Return, Scope,
            VariableDef,
        },
        ZResult,
    },
};

#[derive(Debug, Clone)]
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
    shadowed_vars: Vec<(String, InternalVar)>,

    scope_depth: u32,
    has_main: bool,
    current_labels: Vec<Label>,
}

impl<'guard> Compiler<'guard> {
    pub fn compile(&mut self, source: Mod) -> ZResult<&Module> {
        self.module = Module::new();
        self.builder = Builder::new();

        for node in source.body {
            self.handle_node(node)?;
        }

        if !self.has_main {
            return Err(CompilerError::new(1, 1, 1, "Missing main function."));
        }

        Ok(&self.module)
    }

    fn handle_node(&mut self, node: Node) -> ZResult<()> {
        match node {
            Node::FunctionDef(fun) => {
                let mut f = func::Function::new(&fun.name);

                let return_label = self.builder.get_label();
                self.current_labels.clear();
                self.current_labels.push(return_label);

                let mut offset = 16;
                for arg in fun.args {
                    if let Node::Arg(arg) = arg {
                        let inner = Variable::new(offset);
                        offset += 8;

                        let var = InternalVar::new(inner, false, self.scope_depth);
                        self.vars.insert(arg.name, var);
                    }
                }

                for node in fun.body {
                    self.handle_node(node)?;
                }

                self.builder.write_to_fn(
                    &mut f,
                    self.current_labels
                        .first()
                        .expect("Function return label not found."),
                );
                self.module.add_func(f);

                if fun.name == grammar::F_MAIN {
                    self.has_main = true;
                }
            }
            Node::Scope(scope) => self.build_scope(scope)?,
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
            Node::Break(br) => self.build_break(br)?,
            _ => panic!("Unknown node {:?}", node),
        }

        Ok(())
    }

    fn build_scope(&mut self, scope: Scope) -> ZResult<()> {
        self.add_scope();

        for node in scope.body {
            self.handle_node(node)?;
        }

        self.clear_scope();
        Ok(())
    }

    fn build_if(&mut self, case: If) -> ZResult<()> {
        self.add_scope();
        self.handle_node(*case.test)?;
        self.builder.write_raw("    cmp eax, 1\n");

        let label1 = self.builder.get_label();
        let label2 = self.builder.get_label();

        self.builder.build_jump(&label1, Jump::NotEqual);
        for node in case.run.body {
            self.handle_node(node)?;
        }

        if *case.orelse != Node::None {
            self.builder.build_jump(&label2, Jump::Always);
        }

        self.builder.insert_label(&label1);

        if *case.orelse != Node::None {
            self.handle_node(*case.orelse)?;
            self.builder.insert_label(&label2);
        }

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

        let (unshadowed, shadowed): (_, Vec<_>) = self
            .shadowed_vars
            .clone()
            .into_iter()
            .partition(|e| e.1.scope == self.scope_depth);
        self.shadowed_vars = shadowed;

        for var in unshadowed {
            self.vars.insert(var.0, var.1);
        }
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

    fn build_break(&mut self, pos: SourcePos) -> ZResult<()> {
        let label = match self.current_labels.last() {
            Some(label) => label,
            None => {
                return Err(CompilerError::new(
                    pos.line as usize,
                    pos.column as usize,
                    grammar::BREAK.len(),
                    "Break used outside of loop.",
                ))
            }
        };
        self.builder.build_jump(label, Jump::Always);
        Ok(())
    }

    fn build_inline_asm(&mut self, call: Call) -> ZResult<()> {
        let re = Regex::new(r"\$[A-z]([A-z]|\d)+").expect("Failed to build regular expression.");
        for arg in call.args {
            if let Node::Constant(constant) = arg {
                let text = constant.value.to_string();

                let out = re
                    .replace_all(&text, |caps: &regex::Captures| {
                        let matched_text = &caps[0];
                        match self.vars.get(&matched_text[1..]) {
                            Some(var) => var.inner.clone(),
                            None => return "".to_owned(),
                        }
                        .get_mem_location()
                    })
                    .to_string();

                if out.is_empty() {
                    return Err(CompilerError::new(1, 1, 1, "Bruhhhh"));
                }
                self.builder.write_raw_fmt(&out);
            } else {
                panic!("Only constants can be used in inline asm.")
            }
        }
        Ok(())
    }

    fn build_call(&mut self, mut call: Call) -> ZResult<()> {
        if call.func.id == grammar::F_ASM {
            return Ok(self.build_inline_asm(call))?;
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
        self.builder.build_return(
            operand,
            self.current_labels
                .first()
                .expect("Function return label was not found."),
        );
        Ok(())
    }

    fn build_assign(&mut self, assign: Assign) -> ZResult<()> {
        let value = self.make_operand(*assign.value)?;

        let var = match self.vars.get(&assign.target) {
            Some(var) => var,
            None => {
                return Err(CompilerError::new(
                    assign.pos.line as usize,
                    assign.pos.column as usize,
                    assign.target.len(),
                    &format!("Variable '{}', not found in scope.", assign.target),
                ))
            }
        };
        // checks if var is mutable
        if !var.mutable {
            return Err(CompilerError::new(
                assign.pos.line as usize,
                assign.pos.column as usize,
                assign.target.len(),
                "Variable is imutable.",
            ));
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
        let old = self.vars.insert(
            var.name.clone(),
            InternalVar::new(inner, var.mutable, self.scope_depth),
        );

        if let Some(old) = old {
            self.shadowed_vars.push((var.name, old));
        }

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
            has_main: false,
            shadowed_vars: vec![],
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
            Node::Name(name, pos) => {
                let var = match self.vars.get(&name.id) {
                    Some(var) => var.inner.clone(),
                    None => {
                        return Err(CompilerError::new(
                            pos.line as usize,
                            pos.column as usize,
                            name.id.len(),
                            &format!("Variable '{}' not found in scope.", name.id),
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
