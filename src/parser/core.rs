use super::ast::{Arg, FunctionDef, Module, Name, Node};
use crate::grammar::*;
use crate::tokenizer::token::{Token, Type};

pub struct Parser {
    index: usize,
    tokens: Vec<Token>,
    body: Vec<Node>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            index: 0,
            tokens: vec![],
            body: vec![],
        }
    }

    pub fn build_fun(&mut self) -> FunctionDef {
        self.index += 1;
        let name = match self.getttok(0) {
            Some(name) => {
                if name.t_type != Type::Word {
                    panic!()
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
            let annotation = self.parse_node(&current).unwrap();

            let arg = Arg {
                name,
                annotation: Box::from(annotation),
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
                let returns = self.parse_node(&tok).unwrap();
                definition.returns = Box::from(returns);
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
            let tok = match self.parse_node(&current) {
                Some(tok) => tok,
                None => {
                    self.index += 1;
                    current = self.getttok(0).unwrap();
                    continue;
                }
            };

            self.index += 1;
            current = self.getttok(0).unwrap();

            println!("{:?}", tok);
            definition.body.push(tok);
        }

        self.index += 1;

        definition
    }

    pub fn parse_node(&mut self, tok: &Token) -> Option<Node> {
        match tok.t_type {
            Type::Keyword => match tok.value.as_str() {
                FUN => Some(Node::FunctionDef(self.build_fun())),
                INT => Some(Node::Name(Name { id: INT.to_owned() })),
                FLOAT => Some(Node::Name(Name {
                    id: FLOAT.to_owned(),
                })),
                _ => {
                    println!("{:?}", tok);
                    todo!()
                }
            },
            Type::Nl => None,
            _ => {
                println!("Token failure: {:?}", tok);
                panic!()
            }
        }
    }

    fn step(&mut self) {
        if let Some(token) = self.getttok(0) {
            if let Some(node) = self.parse_node(&token) {
                self.body.push(node);
            }
            self.index += 1;
            self.step();
        }
    }

    fn getttok(&self, offset: isize) -> Option<Token> {
        match self.tokens.get(((self.index as isize) + offset) as usize) {
            Some(tok) => Some(tok.clone()),
            None => None,
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) -> Module {
        let mut module = Module::new();

        self.tokens = tokens;
        self.index = 0;

        self.step();

        module.body = self.body.clone();
        module
    }

    fn throw(&self) {
        panic!()
    }
}
