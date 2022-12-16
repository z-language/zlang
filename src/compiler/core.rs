use crate::parser::ast::{BinOp, Constant, Module, Node, Operator, Primitive};

use super::{instructions::Opcode, Compiler};

macro_rules! inst {
    ($x:expr) => {
        $x as u8
    };
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {}
    }

    fn build_constant(&self, constant: &Constant) -> Result<Vec<u8>, String> {
        let mut buff: Vec<u8> = vec![];

        let value = match constant.value {
            Primitive::Int(i) => {
                if i < 0 {
                    return Err("Negative numbers are not implemented yet.".to_owned());
                }

                if i > 255 {
                    return Err("Const pool not implemented yet.".to_owned());
                }

                buff.push(inst!(Opcode::PUSH));
                i as u8
            }
            Primitive::Float(_) => todo!(),
            Primitive::Str(_) => todo!(),
            Primitive::Bool(_) => todo!(),
        };

        buff.push(value);

        Ok(buff)
    }

    fn build_binop(&self, binop: &BinOp) -> Result<Vec<u8>, String> {
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

    fn parse_node(&self, node: &Node) -> Result<Vec<u8>, String> {
        match node {
            Node::BinOp(nd) => Ok(self.build_binop(&nd)?),
            Node::Constant(nd) => Ok(self.build_constant(&nd)?),
            _ => return Err(format!("Node: {:?} can't be compiled yet.", node)),
        }
    }

    pub fn compile(&self, module: Module) -> Result<Vec<u8>, String> {
        // println!("{:#?}", module);
        let mut buff: Vec<u8> = vec![];
        let mut program = vec![];

        // Version
        buff.push(0x01);

        for node in module.body {
            let bytes = self.parse_node(&node)?;
            program.extend(bytes);
        }
        program.push(inst!(Opcode::DEBUG));

        // Size of const pool
        buff.push(0x00);
        buff.push(0x00);

        // Size of prog
        buff.push(0x00);
        buff.push(program.len() as u8);

        buff.extend(program);

        Ok(buff)
    }
}
