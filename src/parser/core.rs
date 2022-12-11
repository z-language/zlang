use super::ast::{Arg, BinOp, Call, Constant, FunctionDef, Module, Name, Node, Primitive};
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
            None => return Err(CompilerError::new(name.line, name.pos + 1, "Arg error.")),
        };
        while current.t_type != Type::Rparen {
            if current.t_type != Type::Word {
                return Err(CompilerError::new(
                    current.line,
                    current.pos,
                    "Arg name should be a word!",
                ));
            }
            let name = current.value;
            self.index += 1;
            current = match self.getttok(0) {
                Some(tok) => tok,
                None => {
                    return Err(CompilerError::new(
                        current.line,
                        current.pos + 1,
                        "Missing tokens required for argument construction.",
                    ))
                }
            };
            if current.t_type != Type::DoubleDot {
                return Err(CompilerError::new(
                    current.line,
                    current.pos - 1,
                    "Please specify the type of this argument!",
                ));
            }
            self.index += 1;
            current = match self.getttok(0) {
                Some(tok) => tok,
                None => {
                    return Err(CompilerError::new(
                        current.line,
                        current.pos + 1,
                        "Missing tokens required for argument construction.",
                    ))
                }
            };
            let annotation = match self.parse_node(&current)? {
                Some(an) => an,
                None => {
                    return Err(CompilerError::new(
                        current.line,
                        current.pos,
                        "Please provide a type.",
                    ))
                }
            };

            let arg = Arg {
                name,
                annotation: Box::from(annotation),
            };

            definition.args.push(Node::Arg(arg));

            self.index += 1;
            current = self.getttok(0).unwrap();

            if current.t_type == Type::Comma {
                self.index += 1;
                current = match self.getttok(0) {
                    Some(tok) => tok,
                    None => {
                        return Err(CompilerError::new(
                            current.line,
                            current.pos,
                            "Please seperate arguments with a comma.",
                        ))
                    }
                };
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
            None => {
                return Err(CompilerError::new(
                    current.line,
                    current.pos + 1,
                    "Expected a function body!",
                ))
            }
        };

        while current.t_type != Type::Rbrack {
            let tok = self.parse_node(&current)?;

            self.index += 1;
            current = match self.getttok(0) {
                Some(tok) => tok,
                None => {
                    return Err(CompilerError::new(
                        current.line,
                        current.pos,
                        "Expected a closing bracket!",
                    ))
                }
            };

            definition.body.push(match tok {
                Some(tok) => tok,
                None => continue,
            });
        }

        self.index += 1;

        Ok(definition)
    }

    fn build_fcall(&mut self) -> Result<Call, CompilerError> {
        let name = self.getttok(0).unwrap();
        let mut args = vec![];

        self.index += 1;

        // skip the (
        self.index += 1;

        while let Some(token) = self.getttok(0) {
            match token.t_type {
                Type::Comma => {
                    self.index += 1;
                    continue;
                }
                Type::Rparen => break,

                _ => args.push(match self.parse_node(&token)? {
                    Some(tok) => tok,
                    None => continue,
                }),
            };
            self.index += 1;
        }

        Ok(Call {
            func: Name { id: name.value },
            args,
        })
    }

    fn build_constant(&self, tok: &Token) -> Result<Constant, CompilerError> {
        match tok.t_type {
            Type::String => Ok(Constant {
                value: Primitive::Str(tok.value.clone()),
            }),
            Type::Int => {
                let val = tok.value.parse().expect("This shouldn't fail...");

                Ok(Constant {
                    value: Primitive::Int(val),
                })
            }
            Type::Float => {
                let val = tok.value.parse().expect("This shouldn't fail...");

                Ok(Constant {
                    value: Primitive::Float(val),
                })
            }

            _ => {
                return Err(CompilerError::new(
                    tok.line,
                    tok.pos,
                    "Not yet implemented!",
                ))
            }
        }
    }

    fn build_name(&self, tok: &Token) -> Result<Name, CompilerError> {
        if tok.t_type != Type::Word {
            return Err(CompilerError::new(
                tok.line,
                tok.pos,
                "Name should be a Word type!",
            ));
        }
        Ok(Name {
            id: tok.value.clone(),
        })
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
                Ok(Some(Node::Constant(self.build_constant(tok)?)))
            }
            Type::Int | Type::String | Type::Float
                if self.getttok(1).is_some()
                    && self.getttok(1).expect("This shouldn't fail...").t_type == Type::Op =>
            {
                Ok(Some(Node::BinOp(self.build_binop())))
            }

            Type::Word => match self.getttok(1) {
                Some(next_token) => match next_token.t_type {
                    Type::Lparen => return Ok(Some(Node::Call(self.build_fcall()?))),
                    Type::Nl => return Ok(None),
                    _ => return Ok(Some(Node::Name(self.build_name(&tok)?))),
                },
                None => return Ok(None),
            },

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
