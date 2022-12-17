use std::mem::size_of_val;

use crate::parser::ast::{BinOp, Constant, Module, Name, Node, Operator, Primitive, VariableDef};

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
        }
    }

    fn build_constant(&mut self, constant: &Constant) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];

        match constant.value {
            Primitive::Int(i) => {
                if i < 0 {
                    return Err("Negative numbers are not implemented yet.".to_owned());
                }

                if i > 255 {
                    if !self.constants.contains(&constant.value) {
                        self.constants.push(constant.value.clone());
                    }

                    let pos = self
                        .constants
                        .iter()
                        .position(|x| match x {
                            Primitive::Int(x) => x == &i,
                            _ => false,
                        })
                        .expect("This shouldn't fail...") as u8;
                    buff.push(inst!(Opcode::LOAD_CONST));
                    buff.push(pos);
                } else {
                    buff.push(inst!(Opcode::PUSH));
                    buff.push(i as u8);
                }
            }
            Primitive::Float(_) => todo!(),
            Primitive::Str(_) => todo!(),
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

    fn parse_node(&mut self, node: &Node) -> Result<Vec<u8>, String> {
        match node {
            Node::BinOp(nd) => Ok(self.build_binop(nd)?),
            Node::Constant(nd) => Ok(self.build_constant(nd)?),
            Node::VariableDef(nd) => Ok(self.build_var(nd)?),
            Node::Name(nd) => Ok(self.build_name(nd)?),
            _ => return Err(format!("Node: {:?} can't be compiled yet.", node)),
        }
    }

    pub fn compile(&mut self, module: Module) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];
        let mut program = vec![];

        // Version
        buff.push(0x01);

        for node in module.body {
            let bytes = self.parse_node(&node)?;
            program.extend(bytes);
        }
        program.push(inst!(Opcode::DEBUG));
        program.push(inst!(Opcode::HLT));

        // Size of const pool
        let size_of_consts = (self.constants.len() as i16).to_be_bytes();
        buff.extend(size_of_consts);

        for constant in &self.constants {
            let bytes = make_constant(constant);
            buff.extend(bytes);
        }

        // Size of prog
        let size_of_prog = (program.len() as i16).to_be_bytes();
        buff.extend(size_of_prog);

        buff.extend(program);

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
