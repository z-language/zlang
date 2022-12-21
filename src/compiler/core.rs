use std::{collections::HashMap, mem::size_of_val};

use crate::parser::ast::{
    BinOp, Call, Constant, FunctionDef, If, Module, Name, Node, Operator, Primitive, Return, Scope,
    VariableDef,
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
            variable_map: vec![vec![]],
            constants: vec![],
            function_map: HashMap::new(),
            function_store: vec![],
            iteration: 0,
            scope: 0,
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

        Ok(buff)
    }

    fn build_var(&mut self, var: &VariableDef) -> Result<Vec<u8>, String> {
        let mut buff = vec![];

        self.variable_map[self.scope].push(var.name.clone());

        let value = self.parse_node(&var.value)?;
        buff.extend(value);

        buff.push(inst!(Opcode::STORE_NAME));
        let index = self.get_variable_index(&var.name);
        buff.push(index);
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
        };
        buff.push(inst!(op));

        Ok(buff)
    }

    fn build_name(&self, name: &Name) -> Result<Vec<u8>, String> {
        if self.iteration == 0 {
            return Ok(vec![0x00, 0x00]);
        }

        let mut buff = vec![];

        buff.push(inst!(Opcode::LOAD_NAME));
        let index = self.get_variable_index(&name.id);
        buff.push(index);

        Ok(buff)
    }

    fn get_variable_index(&self, name: &str) -> u8 {
        let index = self.variable_map[self.scope]
            .iter()
            .position(|x| *x == name)
            .expect("This shouldn't fail...");

        (index + (self.scope * 10)) as u8
    }

    fn build_fun(&mut self, fun: &FunctionDef) -> Result<Vec<u8>, String> {
        self.scope += 1;
        self.variable_map.push(vec![]);

        let mut func_body = vec![];

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

        self.scope -= 1;
        self.variable_map.pop();
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
        let mut body = self.parse_node(&if_statement.run)?;
        let orelse = self.parse_node(&if_statement.orelse)?;

        body.push(inst!(Opcode::PUSH));
        body.push(orelse.len() as u8);
        body.push(inst!(Opcode::JMPF));

        buff.extend(test);
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
        let mut break_positions = vec![];

        for node in &scope.body {
            match node {
                Node::Break => {
                    buff.push(inst!(Opcode::PUSH));
                    buff.push(0x00);
                    buff.push(inst!(Opcode::JMPF));
                    break_positions.push(buff.len() - 2);
                    continue;
                }
                _ => (),
            };

            let bytes = self.parse_node(node)?;
            buff.extend(bytes);
        }

        let len = buff.len() - 2;

        for pos in break_positions {
            buff[pos] = (len - pos) as u8;
        }

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

        Ok(buff)
    }

    fn build_return(&mut self, ret: &Return) -> Result<Vec<u8>, String> {
        let mut buff = self.parse_node(&ret.value)?;

        buff.push(inst!(Opcode::RETURN));

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
        match node {
            Node::BinOp(nd) => Ok(self.build_binop(nd)?),
            Node::Constant(nd) => Ok(self.build_constant(nd)?),
            Node::VariableDef(nd) => Ok(self.build_var(nd)?),
            Node::Return(nd) => Ok(self.build_return(nd)?),
            Node::Call(nd) => Ok(self.build_call(nd)?),
            Node::Name(nd) => Ok(self.build_name(nd)?),
            Node::If(nd) => Ok(self.build_if(nd)?),
            Node::Scope(nd) => Ok(self.build_scope(nd)?),
            _ => return Err(format!("Node {:?} can't be compiled yet.", node)),
        }
    }

    pub fn compile(&mut self, module: Module) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];
        let mut program = vec![];

        // Version
        buff.push(0x01);

        let funcs: Vec<&FunctionDef> = module
            .body
            .iter()
            .filter(|node| match node {
                Node::FunctionDef(_) => true,
                _ => false,
            })
            .map(|node| match node {
                Node::FunctionDef(fun) => fun,
                _ => (panic!()),
            })
            .collect();

        // Build functions
        funcs.iter().for_each(|fun| self.define_fun(fun));
        for fun in &funcs {
            self.build_fun(fun)?;
        }

        self.iteration += 1;

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
