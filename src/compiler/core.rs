use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    mem::size_of_val,
};

use crate::parser::ast::{
    Assign, BinOp, Call, Constant, FunctionDef, If, Loop, Module, Name, Node, Operator, Primitive,
    Return, Scope, VariableDef,
};

use super::{
    instructions::{Opcode, Type},
    Compiler,
};

macro_rules! inst {
    ($x:expr) => {
        $x as u8
    };
    () => {
        inst!(Opcode::NOOP)
    };
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            constants: vec![],
            function_map: HashMap::new(),
            function_store: vec![],
            iteration: 0,
            loop_store: vec![],
            pos: 0,
            current_func: vec![],
        }
    }

    fn get_constant(&mut self, value: &Primitive) -> u8 {
        if !self.constants.contains(value) {
            self.constants.push(value.clone());
        }

        self.constants
            .iter()
            .position(|x| x == value)
            .expect("This shouldn't fail...") as u8
    }

    fn build_constant(&mut self, constant: &Constant) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];

        match constant.value {
            Primitive::Int(i) => {
                if i < 0 {
                    return Err("Negative numbers are not implemented yet.".to_owned());
                }

                if i > 255 {
                    let pos = self.get_constant(&constant.value);
                    buff.push(inst!(Opcode::LOAD_CONST));
                    buff.push(pos);
                } else {
                    buff.push(inst!(Opcode::PUSH));
                    buff.push(i as u8);
                }
            }
            Primitive::Float(_) => todo!(),
            Primitive::Str(_) => {
                let pos = self.get_constant(&constant.value);
                buff.push(inst!(Opcode::LOAD_CONST));
                buff.push(pos);
            }
            Primitive::Bool(_) => todo!(),
        };

        self.pos += 2;
        Ok(buff)
    }

    fn build_var(&mut self, var: &VariableDef) -> Result<Vec<u8>, String> {
        let mut buff = vec![];

        if matches!(*var.value, Node::None) {
            buff.extend([inst!(Opcode::PUSH), 0]);
        } else {
            let value = self.parse_node(&var.value)?;
            buff.extend(value);
        }

        buff.push(inst!(Opcode::STORE_NAME));
        let index = self.get_variable_index(&var.name);
        buff.push(index);

        self.pos += 2;
        Ok(buff)
    }

    fn build_assign(&mut self, assign: &Assign) -> Result<Vec<u8>, String> {
        let mut buff = vec![];

        let value = self.parse_node(&assign.value)?;
        buff.extend(value);

        buff.push(inst!(Opcode::STORE_NAME));
        let index = self.get_variable_index(&assign.target);
        buff.push(index);

        self.pos = 2;
        Ok(buff)
    }

    fn build_binop(&mut self, binop: &BinOp) -> Result<Vec<u8>, String> {
        let mut buff = vec![];

        buff.extend(self.parse_node(&binop.left)?);
        buff.extend(self.parse_node(&binop.right)?);

        let op = match binop.op {
            Operator::Add => Opcode::ADD,
            Operator::Sub => Opcode::SUB,
            Operator::Mult => Opcode::MUL,
            Operator::Div => Opcode::DIV,
            Operator::DoubleEquals => Opcode::EQ,
            Operator::Mod => Opcode::MOD,
        };
        buff.push(inst!(op));

        self.pos += 1;
        Ok(buff)
    }

    fn build_name(&mut self, name: &Name) -> Result<Vec<u8>, String> {
        if self.iteration == 0 {
            return Ok(vec![0x00, 0x00]);
        }

        let mut buff = vec![];

        buff.push(inst!(Opcode::LOAD_NAME));
        let index = self.get_variable_index(&name.id);
        buff.push(index);

        self.pos += 2;
        Ok(buff)
    }

    fn get_variable_index(&self, name: &str) -> u8 {
        let name = String::from(format!("{}.{}", self.current_func.last().unwrap(), name));
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let hash = hasher.finish();

        (hash % 255) as u8
    }

    fn build_fun(&mut self, fun: &FunctionDef) -> Result<Vec<u8>, String> {
        let mut func_body = vec![];
        self.current_func.push(fun.name.clone());

        for arg in &fun.args {
            let arg_def = match arg {
                Node::Arg(name) => VariableDef {
                    name: name.name.clone(),
                    mutable: false,
                    value: Box::new(Node::Constant(Constant {
                        value: Primitive::Int(0),
                    })),
                },
                _ => panic!(),
            };
            self.build_var(&arg_def)?;
            func_body.extend(vec![
                inst!(Opcode::STORE_NAME),
                self.get_variable_index(&arg_def.name),
            ]);
        }

        let body: Result<Vec<Vec<u8>>, String> =
            fun.body.iter().map(|node| self.parse_node(node)).collect();
        for node in body? {
            func_body.extend(node);
        }
        if fun.name == "main" {
            func_body.push(inst!(Opcode::DEBUG));
            func_body.push(inst!(Opcode::HLT));
        }

        func_body.push(inst!(Opcode::RETURN));

        let func_in_store = self.function_map.get(&fun.name).unwrap().1;

        self.function_store[func_in_store] = func_body;

        self.current_func.pop();
        Ok(vec![])
    }

    fn define_fun(&mut self, fun: &FunctionDef) {
        self.function_store.push(vec![]);
        self.function_map
            .insert(fun.name.clone(), (0, self.function_store.len() - 1));
    }

    fn build_if(&mut self, if_statement: &If) -> Result<Vec<u8>, String> {
        let mut buff = vec![];

        let test = self.parse_node(&if_statement.test)?;
        // later push inst after test
        self.pos += 6;
        let mut body = self.parse_node(&if_statement.run)?;
        self.pos += 3;
        let orelse = self.parse_node(&if_statement.orelse)?;

        body.push(inst!(Opcode::PUSH));
        body.push(orelse.len() as u8);
        body.push(inst!(Opcode::JMPF));

        buff.extend(&test);
        buff.push(inst!(Opcode::PUSH));
        buff.push(0x00);
        buff.push(inst!(Opcode::EQ));
        buff.push(inst!(Opcode::PUSH));
        buff.push(body.len() as u8);
        buff.push(inst!(Opcode::JMPT));
        buff.extend(body);
        buff.extend(orelse);

        Ok(buff)
    }

    fn build_scope(&mut self, scope: &Scope) -> Result<Vec<u8>, String> {
        let mut buff = vec![];

        for node in &scope.body {
            let bytes = self.parse_node(node)?;
            buff.extend(bytes);
        }

        Ok(buff)
    }

    fn build_break(&mut self) -> Vec<u8> {
        let loop_ref = self.loop_store.last().unwrap();
        if loop_ref.1 == 0 {
            return vec![0x00; 3];
        }

        let skip = loop_ref.1 - (self.pos);
        let buff = vec![inst!(Opcode::PUSH), skip as u8, inst!(Opcode::JMPF)];

        self.pos += 3;
        buff
    }

    fn build_loop(&mut self, loop_def: &Loop) -> Result<Vec<u8>, String> {
        self.loop_store.push((0, 0));
        let mut len = 0;

        self.loop_store.last_mut().unwrap().0 = self.pos;

        for node in &loop_def.body {
            len += self.parse_node(node)?.len();
        }

        self.pos = 0;

        self.loop_store.last_mut().unwrap().1 = len;

        let mut buff = vec![];
        for node in &loop_def.body {
            let bytes = self.parse_node(node)?;
            self.loop_store.last_mut().unwrap().0 += bytes.len();
            buff.extend(bytes);
        }

        buff.push(inst!(Opcode::PUSH));
        buff.push(buff.len() as u8 + 2);
        buff.push(inst!(Opcode::JMPB));

        self.loop_store.pop();
        self.pos += 3;
        Ok(buff)
    }

    fn build_call(&mut self, call: &Call) -> Result<Vec<u8>, String> {
        let mut buff = vec![];
        let args = &call.args.clone();
        for arg in args.iter().rev() {
            let arg_parsed = self.parse_node(arg)?;
            buff.extend(arg_parsed);
        }

        if self.iteration == 0 {
            let filler_bytes = 2 + buff.len();
            return Ok(vec![0x00; filler_bytes]);
        }

        buff.push(inst!(Opcode::CALL));
        let called_fun = self.function_map.get(&call.func.id).unwrap().0;
        let constant = self.get_constant(&Primitive::Int(called_fun as i32));
        buff.push(constant);

        self.pos += 2;
        Ok(buff)
    }

    fn build_return(&mut self, ret: &Return) -> Result<Vec<u8>, String> {
        let mut buff = self.parse_node(&ret.value)?;

        buff.push(inst!(Opcode::RETURN));

        self.pos += 1;
        Ok(buff)
    }

    fn compile_functions(&mut self, buff_len: usize) -> Vec<u8> {
        let mut vec = vec![];

        for func in self.function_map.clone() {
            let values = func.1;
            let pos = vec.len() + buff_len + 1;
            self.function_map
                .entry(func.0)
                .and_modify(|funct| funct.0 = pos);

            let bytes = &self.function_store[values.1];
            vec.extend(bytes);
        }

        vec
    }

    fn parse_node(&mut self, node: &Node) -> Result<Vec<u8>, String> {
        let bytes = match node {
            Node::BinOp(nd) => self.build_binop(nd)?,
            Node::Constant(nd) => self.build_constant(nd)?,
            Node::VariableDef(nd) => self.build_var(nd)?,
            Node::Assign(nd) => self.build_assign(nd)?,
            Node::Return(nd) => self.build_return(nd)?,
            Node::Call(nd) => self.build_call(nd)?,
            Node::Name(nd) => self.build_name(nd)?,
            Node::If(nd) => self.build_if(nd)?,
            Node::Scope(nd) => self.build_scope(nd)?,
            Node::Loop(nd) => self.build_loop(nd)?,
            Node::None => vec![],
            Node::Break => self.build_break(),
            _ => return Err(format!("Node {:?} can't be compiled yet.", node)),
        };

        Ok(bytes)
    }

    pub fn compile(&mut self, module: Module) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];
        let mut program = vec![];

        // Version
        buff.push(0x01);

        let funcs: Vec<&FunctionDef> = module
            .body
            .iter()
            .filter(|node| matches!(node, Node::FunctionDef(_)))
            .map(|node| match node {
                Node::FunctionDef(fun) => fun,
                _ => panic!(),
            })
            .collect();

        // Build functions
        funcs.iter().for_each(|fun| self.define_fun(fun));
        for fun in &funcs {
            self.build_fun(fun)?;
        }

        self.iteration += 1;
        self.pos = 0;

        // We compile them just to get the size of all functions.
        self.compile_functions(buff.len());

        for fun in funcs {
            self.build_fun(fun)?;
        }

        self.iteration += 1;

        // Now we recompile all functions and append them to the program.
        program.extend(self.compile_functions(buff.len()));

        // Call the main func
        program.insert(0, inst!(Opcode::CALL));
        let main_func = self.function_map.get("main").unwrap();
        program.insert(1, self.get_constant(&Primitive::Int(main_func.0 as i32)));

        // Make program
        for node in module.body {
            match node {
                Node::FunctionDef(_) => continue,
                _ => (),
            }
            let bytes = self.parse_node(&node)?;
            program.extend(bytes);
        }

        // Size of prog
        let size_of_prog = (program.len() as i16).to_be_bytes();
        buff.insert(1, size_of_prog[1]);
        buff.insert(1, size_of_prog[0]);

        // Append the whole program
        buff.extend(program);
        // buff.extend(vec![inst!(), inst!(), inst!(), inst!(), inst!(), inst!()]);

        // Size of const pool
        let size_of_consts = (self.constants.len() as i16).to_be_bytes();
        buff.extend(size_of_consts);

        // Append all constants
        for constant in &self.constants {
            let bytes = make_constant(constant);
            buff.extend(bytes);
        }

        Ok(buff)
    }
}

fn make_constant(c: &Primitive) -> Vec<u8> {
    let mut buff = vec![];

    match c {
        Primitive::Int(i) => {
            buff.push(inst!(Type::T_INT));
            buff.push(size_of_val(i) as u8);
            let mut bytes = i.to_be_bytes();
            bytes.reverse();
            buff.extend(bytes);
        }
        Primitive::Float(_) => todo!(),
        Primitive::Str(i) => {
            let mut value = i.clone();
            value.push('\0');
            buff.push(inst!(Type::T_STR));
            let size = value.len() as u8;
            buff.push(size);
            let bytes = (value.into_bytes()).to_vec();
            buff.extend(bytes);
        }
        Primitive::Bool(_) => todo!(),
    }

    buff
}
