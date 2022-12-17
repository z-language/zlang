use std::{collections::HashMap, mem::size_of_val};

use crate::parser::ast::{
    BinOp, Constant, FunctionDef, Module, Name, Node, Operator, Primitive, VariableDef,
};

use super::{
    instructions::{Opcode, Type},
    Compiler,
};

macro_rules! inst {
    ($x:expr) => {
        $x as u8
    };
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            variable_map: vec![],
            constants: vec![],
            function_map: HashMap::new(),
            function_store: vec![],
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

        self.variable_map.push(var.name.clone());

        let value = self.parse_node(&var.value)?;
        buff.extend(value);

        buff.push(inst!(Opcode::STORE_NAME));
        let index = self
            .variable_map
            .iter()
            .position(|x| *x == var.name)
            .expect("This shouldn't fail...") as u8;
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
        };
        buff.push(inst!(op));

        Ok(buff)
    }

    fn build_name(&self, name: &Name) -> Result<Vec<u8>, String> {
        let mut buff = vec![];

        buff.push(inst!(Opcode::LOAD_NAME));
        let index = self
            .variable_map
            .iter()
            .position(|x| *x == name.id)
            .expect("This shouldn't fail...") as u8;
        buff.push(index);

        Ok(buff)
    }

    fn build_fun(&mut self, fun: &FunctionDef) -> Result<Vec<u8>, String> {
        let mut func_body = vec![];

        let body: Result<Vec<Vec<u8>>, String> =
            fun.body.iter().map(|node| self.parse_node(node)).collect();
        for node in body? {
            func_body.extend(node);
        }
        if fun.name == "main" {
            func_body.push(inst!(Opcode::DEBUG));
            func_body.push(inst!(Opcode::HLT));
        }
        self.function_store.push(func_body);
        self.function_map
            .insert(fun.name.clone(), (0, self.function_store.len() - 1));

        Ok(vec![])
    }

    fn parse_node(&mut self, node: &Node) -> Result<Vec<u8>, String> {
        match node {
            Node::BinOp(nd) => Ok(self.build_binop(nd)?),
            Node::Constant(nd) => Ok(self.build_constant(nd)?),
            Node::VariableDef(nd) => Ok(self.build_var(nd)?),
            Node::FunctionDef(nd) => Ok(self.build_fun(nd)?),
            Node::Name(nd) => Ok(self.build_name(nd)?),
            _ => return Err(format!("Node: {:?} can't be compiled yet.", node)),
        }
    }

    pub fn compile(&mut self, module: Module) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];
        let mut program = vec![];

        // Version
        buff.push(0x01);

        // Build functions
        for node in &module.body {
            match node {
                Node::FunctionDef(_) => self.parse_node(node)?,
                _ => continue,
            };
        }

        for func in self.function_map.clone() {
            let values = func.1;
            // Plus two because the program length (2 bytes) and first ins (2 bytes) are inserted later
            let pos = program.len() + buff.len() + 1;
            self.function_map
                .entry(func.0)
                .and_modify(|funct| funct.0 = pos);

            let bytes = &self.function_store[values.1];
            program.extend(bytes);
        }

        // Call the main func
        program.insert(0, inst!(Opcode::CALL));
        let main_func = self.function_map.get("main").unwrap();
        program.insert(1, self.get_constant(&Primitive::Int(main_func.0 as i32)));
        println!("{:?}", self.function_map);

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
        Primitive::Str(_) => todo!(),
        Primitive::Bool(_) => todo!(),
    }

    buff
}
