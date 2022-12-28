use super::ast::{
    Arg, Assign, BinOp, Call, Constant, FunctionDef, If, Loop, Module, Name, Node, Operator,
    Primitive, Return, Scope, VariableDef,
};
use super::Parser;
use crate::error::{CompilerError, MakeErr};
use crate::grammar::*;
use crate::parser::rpn::shutting_yard;
use crate::tokenizer::token::{Token, Type};

#[derive(Debug, PartialEq)]
pub enum ExprPart {
    Operator(Operator),
    Operand(Node),
    Lpar,
    Rpar,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            index: 0,
            tokens: vec![],
            body: vec![],
            building_binop: vec![false],
            scope: 0,
        }
    }

    fn build_fun(&mut self) -> Result<FunctionDef, CompilerError> {
        self.index += 1;
        let name = match self.gettok(0) {
            Some(name) => {
                if name.t_type != Type::Word {
                    return Err(name.into_err("Expected a word for function name."));
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
        let mut current = match self.gettok(0) {
            Some(tok) => tok,
            None => return Err(name.into_err("Arg error.")), // + 1
        };
        while current.t_type != Type::Rparen {
            if current.t_type != Type::Word {
                return Err(current.into_err("Arg name should be a word!"));
            }
            let name = current.value;
            self.index += 1;
            current = match self.gettok(0) {
                Some(tok) => tok,
                None => {
                    return Err(
                        CompilerError::new(0, 0, 0, "a"), // TODO: current.into_err("Missing tokens required for argument construction.")
                    );
                }
            };
            if current.t_type != Type::DoubleDot {
                return Err(current.into_err("Please specify the type of this argument!"));
            }
            self.index += 1;
            current = match self.gettok(0) {
                Some(tok) => tok,
                None => {
                    return Err(
                        current.into_err("Missing tokens required for argument construction.")
                    )
                }
            };
            if current.t_type == Type::Rparen {
                return Err(current.into_err("Please specify a type!"));
            }
            let annotation = match self.parse_node(&current)? {
                Some(an) => match &an {
                    Node::Name(_) => an,
                    _ => return Err(current.into_err("This isn't a valid type!")),
                },
                None => return Err(current.into_err("Please provide a type.")),
            };

            let arg = Arg {
                name,
                annotation: Box::from(annotation),
            };

            definition.args.push(Node::Arg(arg));

            self.index += 1;
            current = self.gettok(0).unwrap();

            if current.t_type == Type::Comma {
                self.index += 1;
                current = match self.gettok(0) {
                    Some(tok) => tok,
                    None => return Err(current.into_err("Please seperate arguments with a comma.")),
                };
            }
        }
        // skip )
        self.index += 1;

        if let Some(tok) = self.gettok(0) {
            if tok.t_type == Type::Arrow {
                self.index += 1;
                let tok = self.gettok(0).unwrap();
                match tok.t_type {
                    Type::Lbrack => {
                        return Err(current.into_err_offset(-1, "Expected a return type!"))
                    }
                    _ => (),
                }
                let returns = self.parse_node(&tok)?.expect("This shouldn't fail...");
                match returns {
                    Node::Name(_) => (),
                    _ => return Err(tok.into_err("This isn't a valid type!")),
                }

                definition.returns = Box::from(returns);
                self.index += 1;
            }
        }

        // skip the { token
        self.index += 1;

        let mut current = match self.gettok(0) {
            Some(tok) => tok,
            None => return Err(current.into_err_offset(1, "Expected a function body!")),
        };

        while current.t_type != Type::Rbrack {
            let tok = self.parse_node(&current)?;

            self.index += 1;
            current = match self.gettok(0) {
                Some(tok) => tok,
                None => return Err(current.into_err("Expected a closing bracket!")),
            };

            definition.body.push(match tok {
                Some(tok) => tok,
                None => continue,
            });
        }

        Ok(definition)
    }

    fn build_fcall(&mut self) -> Result<Call, CompilerError> {
        let name = self.gettok(0).unwrap();
        let mut args = vec![];

        self.building_binop.push(false);
        self.scope += 1;

        self.index += 1;

        // skip the (
        self.index += 1;

        while let Some(token) = self.gettok(0) {
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

        self.building_binop.pop().expect("This shouldn't fail.");
        self.scope -= 1;

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
            Type::Keyword => match tok.value.as_str() {
                "true" => Ok(Constant {
                    value: Primitive::Bool(true),
                }),
                "false" => Ok(Constant {
                    value: Primitive::Bool(false),
                }),
                _ => panic!(),
            },

            _ => return Err(tok.into_err("Not yet implemented!")),
        }
    }

    fn build_var(&mut self) -> Result<VariableDef, CompilerError> {
        self.index += 1;

        let mut mutable = false;
        if let Some(tok) = self.gettok(0) {
            if tok.t_type == Type::Keyword && tok.value == "mut" {
                mutable = true;
                self.index += 1;
            }
        }

        let name = match self.gettok(0) {
            Some(name) => {
                if name.t_type != Type::Word {
                    return Err(name.into_err("Variable name should be a word!"));
                }
                name
            }
            None => {
                let tok = self.gettok(-1).expect("This shouldn't fail");
                return Err(tok.into_err_offset(1, "Expected a variable decleration."));
            }
        };

        // skip the name token
        self.index += 1;

        let mut assigning = false;
        if let Some(token) = self.gettok(0) {
            if token.t_type == Type::Equals {
                self.index += 1;
                assigning = true;
            } else {
                if !mutable {
                    return Err(
                        name.into_err("Immutable variables have to be assigned at declaration.")
                    );
                }
            }
        } else {
            if !mutable {
                return Err(
                    name.into_err("Immutable variables have to be assigned at declaration.")
                );
            }
        }

        let value = if !assigning {
            self.index -= 1;
            Node::None
        } else {
            match self.gettok(0) {
                Some(val) => match self.parse_node(&val)? {
                    Some(val) => val,
                    None => return Err(val.into_err("Expected a value in variable assignment.")),
                },
                None => return Err(name.into_err("Expected a value!")),
            }
        };

        Ok(VariableDef {
            name: name.value,
            mutable,
            value: Box::new(value),
        })
    }

    fn build_assign(&mut self, tok: &Token) -> Result<Assign, CompilerError> {
        // skip the word and the = tokens
        self.index += 2;

        let value = match self.gettok(0) {
            Some(tok) => tok,
            None => return Err(tok.into_err("Expected a value.")),
        };

        Ok(Assign {
            target: tok.value.clone(),
            value: Box::new(match self.parse_node(&value)? {
                Some(tok) => tok,
                None => return Err(tok.into_err("Expected a value.")),
            }),
        })
    }

    fn build_name(&self, tok: &Token) -> Result<Name, CompilerError> {
        if tok.t_type != Type::Word {
            return Err(tok.into_err("Name should be a Word type!"));
        }
        Ok(Name {
            id: tok.value.clone(),
        })
    }

    fn build_binop(&mut self) -> Result<BinOp, CompilerError> {
        let mut expr_unordered: Vec<ExprPart> = vec![];
        self.building_binop[self.scope] = true;

        let mut need_closing = 0;
        while let Some(token) = self.gettok(0) {
            match token.t_type {
                Type::Op => {
                    expr_unordered.push(ExprPart::Operator(match token.value.as_str() {
                        "+" => Operator::Add,
                        "-" => Operator::Sub,
                        "*" => Operator::Mult,
                        "/" => Operator::Div,
                        "==" => Operator::DoubleEquals,
                        "%" => Operator::Mod,
                        _ => return Err(token.into_err("Unknown token.")),
                    }));
                    self.index += 1;
                    continue;
                }
                Type::Int | Type::String | Type::Float | Type::Word => (),
                Type::Lparen => {
                    need_closing += 1;
                    expr_unordered.push(ExprPart::Lpar);
                    self.index += 1;
                    continue;
                }
                Type::Rparen => {
                    if need_closing == 0 {
                        break;
                    }
                    expr_unordered.push(ExprPart::Rpar);
                    self.index += 1;
                    continue;
                }

                _ => break,
            };
            let tok = self.parse_node(&token)?.expect("This shouldn't fail.");

            self.index += 1;
            expr_unordered.push(ExprPart::Operand(tok));
        }

        // 1 2 fcall() - * 4 /
        // (1 * (2 - fcall())) - 4

        // R = 2 - fcall()
        // R2 = 1 * R
        // R3 = R2 - 4

        let mut expr_ordered = shutting_yard(expr_unordered)?;
        expr_ordered.reverse();

        let mut stack: Vec<Node> = vec![];
        while let Some(part) = expr_ordered.pop() {
            match part {
                ExprPart::Operator(op) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Node::BinOp(BinOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    }))
                }
                ExprPart::Operand(operand) => stack.push(operand),

                other => {
                    println!("Unexpected token: {:?}", other);
                    panic!()
                }
            }
        }

        self.index -= 1;
        self.building_binop[self.scope] = false;
        Ok(match stack.pop().unwrap() {
            Node::BinOp(binop) => binop,
            _ => panic!(),
        })
    }

    fn build_scope(&mut self, tok: &Token) -> Result<Scope, CompilerError> {
        // skip {
        self.index += 1;

        let mut scope = Scope { body: vec![] };

        let mut current = match self.gettok(0) {
            Some(tok) => tok,
            None => return Err(tok.into_err_offset(1, "Expected a scope body!")),
        };

        while current.t_type != Type::Rbrack {
            let tok = self.parse_node(&current)?;

            self.index += 1;
            current = match self.gettok(0) {
                Some(tok) => tok,
                None => return Err(current.into_err("Expected a closing bracket!")),
            };

            scope.body.push(match tok {
                Some(tok) => tok,
                None => continue,
            });
        }

        Ok(scope)
    }

    fn build_if(&mut self) -> Result<If, CompilerError> {
        self.index += 1;
        let test = self.parse_node(&self.gettok(0).unwrap())?.unwrap();

        self.index += 1;
        let run = self.parse_node(&self.gettok(0).unwrap())?.unwrap();

        let mut if_statement = If {
            test: Box::new(test),
            run: Box::new(run),
            orelse: Box::new(Node::None),
        };

        self.index += 1;
        let mut token = self.gettok(0).unwrap();
        while token.t_type == Type::Nl {
            self.index += 1;
            token = self.gettok(0).unwrap();
        }
        if token.value == "else" {
            self.index += 1;
            let orelse = self.parse_node(&self.gettok(0).unwrap())?.unwrap();
            if_statement.orelse = Box::new(orelse);
        } else {
            self.index -= 1;
        }

        Ok(if_statement)
    }

    fn build_loop(&mut self) -> Result<Loop, CompilerError> {
        self.index += 2;

        let mut current = self.gettok(0).unwrap();

        let mut body = vec![];

        while current.t_type != Type::Rbrack {
            let tok = self.parse_node(&current)?;

            self.index += 1;
            current = match self.gettok(0) {
                Some(tok) => tok,
                None => return Err(current.into_err("Expected a closing bracket!")),
            };

            body.push(match tok {
                Some(tok) => tok,
                None => continue,
            });
        }

        Ok(Loop { body })
    }

    fn build_return(&mut self, tok: &Token) -> Result<Return, CompilerError> {
        self.index += 1;

        let token = match self.gettok(0) {
            Some(tok) => tok,
            None => return Err(tok.into_err("This return doesn't return anything.")),
        };

        let value = match self.parse_node(&token)? {
            Some(ret) => ret,
            None => return Err(tok.into_err("This return doesn't return anything.")),
        };

        Ok(Return {
            value: Box::new(value),
        })
    }

    fn parse_node(&mut self, tok: &Token) -> Result<Option<Node>, CompilerError> {
        match tok.t_type {
            Type::Keyword => match tok.value.as_str() {
                FUN => Ok(Some(Node::FunctionDef(self.build_fun()?))),
                INT => Ok(Some(Node::Name(Name { id: INT.to_owned() }))),
                FLOAT => Ok(Some(Node::Name(Name {
                    id: FLOAT.to_owned(),
                }))),
                RETURN => Ok(Some(Node::Return(self.build_return(tok)?))),
                TRUE | FALSE => Ok(Some(Node::Constant(self.build_constant(tok)?))),
                VAR => Ok(Some(Node::VariableDef(self.build_var()?))),
                IF => Ok(Some(Node::If(self.build_if()?))),
                LOOP => Ok(Some(Node::Loop(self.build_loop()?))),
                BREAK => {
                    self.index += 1;
                    Ok(Some(Node::Break))
                }
                _ => Err(tok.into_err("Not impl.")),
            },
            Type::Int | Type::String | Type::Float
                if self.gettok(1).is_none()
                    || self.building_binop[self.scope]
                    || self.gettok(1).expect("This shouldn't fail...").t_type != Type::Op =>
            {
                Ok(Some(Node::Constant(self.build_constant(tok)?)))
            }
            Type::Int | Type::String | Type::Float | Type::Word
                if self.gettok(1).is_some()
                    && !self.building_binop[self.scope]
                    && self.gettok(1).expect("This shouldn't fail...").t_type == Type::Op =>
            {
                Ok(Some(Node::BinOp(self.build_binop()?)))
            }
            Type::Lbrack => Ok(Some(Node::Scope(self.build_scope(tok)?))),
            Type::Lparen if !self.building_binop[self.scope] => {
                Ok(Some(Node::BinOp(self.build_binop()?)))
            }

            Type::Word => match self.gettok(1) {
                Some(next_token) => match next_token.t_type {
                    Type::Lparen => {
                        let prev_index = self.index;
                        let call = self.build_fcall()?;
                        let next = match self.gettok(1) {
                            Some(tok) => tok,
                            None => return Ok(Some(Node::Call(call))),
                        };
                        if next.value == "+" && !self.building_binop[self.scope] {
                            self.index = prev_index;
                            return Ok(Some(Node::BinOp(self.build_binop()?)));
                        }
                        return Ok(Some(Node::Call(call)));
                    }
                    Type::Equals => return Ok(Some(Node::Assign(self.build_assign(tok)?))),
                    _ => return Ok(Some(Node::Name(self.build_name(&tok)?))),
                },
                None => return Ok(Some(Node::Name(self.build_name(&tok)?))),
            },

            Type::Nl => Ok(None),
            _ => Err(tok.into_err(&*format!("Unknown token: {}", tok))),
        }
    }

    fn step(&mut self) -> Result<(), CompilerError> {
        if let Some(token) = self.gettok(0) {
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

    fn gettok(&self, offset: isize) -> Option<Token> {
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
