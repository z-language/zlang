use super::ast::{Arg, BinOp, Constant, FunctionDef, Module, Name, Node, Primitive};
use super::Parser;
use crate::error::CompilerError;
use crate::grammar::*;
use crate::tokenizer::token::{Token, Type};

impl Parser {
    pub fn new() -> Self {
        Parser {
            index: 0,
            tokens: vec![],
            body: vec![],
        }
    }

    fn build_fun(&mut self) -> Result<FunctionDef, CompilerError> {
        self.index += 1;
        let name = match self.getttok(0) {
            Some(name) => {
                if name.t_type != Type::Word {
                    return Err(CompilerError::new(
                        name.line,
                        name.pos,
                        "Expected a word for function name.",
                    ));
                }
                name
            }
            None => {
                panic!()
            }
        };

        let mut definition = FunctionDef {
            name: name.value.clone(),
            args: vec![],
            body: vec![],
            returns: Box::from(Node::None),
        };
        self.index += 1;

        // skip (
        self.index += 1;
        // TODO: code cleanup
        let mut current = match self.getttok(0) {
            Some(tok) => tok,
            None => todo!(),
        };
        while current.t_type != Type::Rparen {
            if current.t_type != Type::Word {
                todo!()
            }
            let name = current.value;
            self.index += 1;
            current = self.getttok(0).unwrap();
            if current.t_type != Type::DoubleDot {
                todo!()
            }
            self.index += 1;
            current = self.getttok(0).unwrap();
            let annotation = self.parse_node(&current)?;

            let arg = Arg {
                name,
                annotation: Box::from(annotation.unwrap()),
            };

            definition.args.push(Node::Arg(arg));

            self.index += 1;
            current = self.getttok(0).unwrap();

            if current.t_type == Type::Comma {
                self.index += 1;
                current = self.getttok(0).unwrap();
            }
        }
        // skip )
        self.index += 1;

        if let Some(tok) = self.getttok(0) {
            if tok.t_type == Type::Arrow {
                self.index += 1;
                let tok = self.getttok(0).unwrap();
                let returns = self.parse_node(&tok)?;
                definition.returns = Box::from(returns.unwrap());
                self.index += 1;
            }
        }

        // skip the { token
        self.index += 1;

        let mut current = match self.getttok(0) {
            Some(tok) => tok,
            None => todo!(),
        };

        while current.t_type != Type::Rbrack {
            let tok = self.parse_node(&current)?;

            self.index += 1;
            current = self.getttok(0).unwrap();

            definition.body.push(match tok {
                Some(tok) => tok,
                None => continue,
            });
        }

        self.index += 1;

        Ok(definition)
    }

    fn build_constant(&self) -> Constant {
        let tok = self.getttok(0).unwrap();
        match tok.t_type {
            Type::String => Constant {
                value: Primitive::Str(tok.value),
            },
            Type::Int => {
                let val = tok.value.parse().expect("This shouldn't fail...");

                Constant {
                    value: Primitive::Int(val),
                }
            }
            Type::Float => {
                let val = tok.value.parse().expect("This shouldn't fail...");

                Constant {
                    value: Primitive::Float(val),
                }
            }

            _ => todo!(),
        }
    }

    fn build_binop(&self) -> BinOp {
        BinOp {
            left: todo!(),
            op: todo!(),
            right: todo!(),
        }
    }

    fn parse_node(&mut self, tok: &Token) -> Result<Option<Node>, CompilerError> {
        match tok.t_type {
            Type::Keyword => match tok.value.as_str() {
                FUN => Ok(Some(Node::FunctionDef(self.build_fun()?))),
                INT => Ok(Some(Node::Name(Name { id: INT.to_owned() }))),
                FLOAT => Ok(Some(Node::Name(Name {
                    id: FLOAT.to_owned(),
                }))),
                _ => Err(CompilerError::new(tok.line, tok.pos, "Not impl.")),
            },
            Type::Int | Type::String | Type::Float
                if self.getttok(1).is_none()
                    || self.getttok(1).expect("This shouldn't fail...").t_type != Type::Op =>
            {
                Ok(Some(Node::Constant(self.build_constant())))
            }
            Type::Int | Type::String | Type::Float
                if self.getttok(1).is_some()
                    && self.getttok(1).expect("This shouldn't fail...").t_type == Type::Op =>
            {
                Ok(Some(Node::BinOp(self.build_binop())))
            }

            Type::Nl => Ok(None),
            _ => Err(CompilerError::new(
                tok.line,
                tok.pos,
                &*format!("Unknown token: {}", tok),
            )),
        }
    }

    fn step(&mut self) -> Result<(), CompilerError> {
        if let Some(token) = self.getttok(0) {
            match self.parse_node(&token) {
                Ok(node) => {
                    if let Some(node) = node {
                        self.body.push(node)
                    }
                }
                Err(err) => return Err(err),
            }
            self.index += 1;
            return self.step();
        };
        Ok(())
    }

    fn getttok(&self, offset: isize) -> Option<Token> {
        match self.tokens.get(((self.index as isize) + offset) as usize) {
            Some(tok) => Some(tok.clone()),
            None => None,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Module, CompilerError> {
        let mut module = Module::new();

        self.tokens = tokens;
        self.index = 0;

        self.step()?;

        module.body = self.body.clone();
        Ok(module)
    }
}
