use super::ast::{
    Arg, Assign, BinOp, Call, Constant, FunctionDef, If, Loop, Module, Name, Node, Operator,
    Primitive, Return, Scope, VariableDef,
};
use super::Parser;
use crate::error::{CompilerError, MakeErr};
use crate::lexer::token::{Keyword, Token, Type};
use crate::lexer::Lexer;
use crate::parser::rpn::shutting_yard;

macro_rules! next {
    ($self:ident) => {
        match $self.tokens.next() {
            Some(val) => val?,
            None => return Err(CompilerError::new(1, 1, 1, "Missing tokens.")),
        }
    };
}

macro_rules! peek {
    ($self:ident) => {
        match $self.tokens.peek() {
            Some(val) => val.clone()?,
            None => return Err(CompilerError::new(1, 1, 1, "Missing tokens.")),
        }
    };
}

#[derive(Debug, PartialEq)]
pub enum ExprPart {
    Operator(Operator),
    Operand(Node),
    Lpar,
    Rpar,
}

impl<'guard> Parser<'guard> {
    pub fn new() -> Self {
        Parser {
            tokens: Lexer::default().peekable(),
            scope: 0,
            building_binop: vec![],
        }
    }

    pub fn parse(&mut self, lexer: Lexer<'guard>) -> Result<Module, CompilerError> {
        let mut module = Module::new();

        self.tokens = lexer.peekable();
        self.scope = 0;
        self.building_binop.clear();
        while let Some(current) = self.tokens.next() {
            let parsed = self.parse_node(current?)?;
            module.body.push(parsed);
        }

        Ok(module)
    }
}

impl<'guard> Parser<'guard> {
    fn parse_node(&mut self, tok: Token) -> Result<Node, CompilerError> {
        match tok.value {
            Type::Primitive(_) if matches!(peek!(self).value, Type::Op(_)) => {
                Ok(Node::BinOp(self.build_binop(tok)?))
            }
            Type::Primitive(_) => Ok(Node::Constant(self.build_constant(tok)?)),
            Type::Keyword(kw) => match kw {
                Keyword::True => todo!(),
                Keyword::False => todo!(),
                Keyword::Mut => todo!(),
                Keyword::Fun => Ok(Node::FunctionDef(self.build_fun()?)),
                Keyword::Var => Ok(Node::VariableDef(self.build_var()?)),
                Keyword::If => Ok(Node::If(self.build_if()?)),
                Keyword::Else => todo!(),
                Keyword::Break => Ok(Node::Break),
                Keyword::Loop => Ok(Node::Loop(self.build_loop()?)),
                Keyword::Return => Ok(Node::Return(self.build_return()?)),
            },

            Type::Word(ref word) => match peek!(self).value {
                Type::LParen => {
                    next!(self);
                    Ok(Node::Call(self.build_fcall(tok)?))
                }
                Type::Equals => {
                    next!(self);
                    Ok(Node::Assign(self.build_assign(tok)?))
                }
                _ => Ok(Node::Name(Name { id: word.clone() })),
            },

            Type::LBrace => Ok(Node::Scope(self.build_scope()?)),

            Type::Nl => {
                let token = next!(self);
                return self.parse_node(token);
            }
            _ => {
                return Err(tok
                    .clone()
                    .into_err(&*format!("Unexpected token: {:?}", tok)))
            }
        }
    }

    fn build_fun(&mut self) -> Result<FunctionDef, CompilerError> {
        let mut current = next!(self);
        let name = if let Type::Word(word) = current.value {
            word
        } else {
            return Err(current.into_err("Function name should be a word."));
        };

        current = next!(self);
        if current.value != Type::LParen {
            return Err(current.into_err("Expected a LParen token."));
        }
        let mut args = vec![];
        current = next!(self);
        while current.value != Type::RParen {
            if current.value == Type::Comma {
                current = next!(self);
                continue;
            }

            let name = if let Type::Word(name) = current.value {
                name
            } else {
                return Err(current.into_err("bruh"));
            };

            current = next!(self);
            if !matches!(current.value, Type::DoubleDot) {
                return Err(current.into_err("bruh"));
            }

            current = next!(self);
            // TODO: limit annotation types
            let annotation = self.parse_node(current)?;

            let arg = Arg {
                name,
                annotation: Box::new(annotation),
            };
            args.push(Node::Arg(arg));

            current = next!(self);
        }

        current = next!(self);

        let mut returns = Node::None;
        if current.value == Type::Arrow {
            current = next!(self);
            if !matches!(current.value, Type::Word(_)) {
                return Err(current.into_err("Function return type should be a word."));
            }
            returns = self.parse_node(current)?;
        } else if current.value != Type::LBrace {
            println!("{:?}", current);
            return Err(current.into_err("Expected a code block."));
        }

        let mut body = vec![];
        current = next!(self);
        while current.value != Type::RBrace {
            if current.value != Type::Nl {
                let parsed = self.parse_node(current)?;
                body.push(parsed);
            }

            current = next!(self);
        }

        Ok(FunctionDef {
            name,
            args,
            body,
            returns: Box::new(returns),
        })
    }

    fn build_fcall(&mut self, name: Token) -> Result<Call, CompilerError> {
        let func = if let Type::Word(name) = name.value {
            Name { id: name }
        } else {
            panic!()
        };

        let mut args = vec![];

        let mut current = next!(self);

        while current.value != Type::RParen {
            if current.value == Type::Comma {
                current = next!(self);
                continue;
            }

            if current.value != Type::Nl {
                args.push(self.parse_node(current)?);
            }
            current = next!(self);
        }

        Ok(Call { func, args })
    }

    fn build_constant(&self, tok: Token) -> Result<Constant, CompilerError> {
        let value: Primitive = match &tok.value {
            Type::Primitive(val) => val.clone(),
            Type::Keyword(val) => match val {
                Keyword::True => Primitive::Bool(true),
                Keyword::False => Primitive::Bool(false),
                _ => panic!(),
            },

            _ => return Err(tok.into_err("Not yet implemented!")),
        };

        Ok(Constant { value })
    }

    fn build_var(&mut self) -> Result<VariableDef, CompilerError> {
        let mut mutable = false;
        let mut current = next!(self);

        if let Type::Keyword(kw) = &current.value {
            if *kw == Keyword::Mut {
                mutable = true;
                current = next!(self);
            }
        }

        let name = if let Type::Word(name) = current.value {
            name
        } else {
            return Err(current.into_err("Variable name should be a word!"));
        };
        current = next!(self);

        let mut assigning = false;

        if current.value == Type::Equals {
            assigning = true;
        } else {
            if !mutable {
                return Err(
                    current.into_err("Immutable variables have to be assigned at declaration.")
                );
            }
        }

        let value = if !assigning {
            Node::None
        } else {
            current = next!(self);
            if current.value == Type::Nl {
                return Err(current.into_err("Expected a value!"));
            }

            self.parse_node(current)?
        };

        Ok(VariableDef {
            name,
            mutable,
            value: Box::new(value),
        })
    }

    fn build_assign(&mut self, name: Token) -> Result<Assign, CompilerError> {
        let current = next!(self);
        if current.value == Type::Nl {
            return Err(current.into_err("Expected a value."));
        }

        let target = if let Type::Word(target) = name.value {
            target
        } else {
            return Err(name.into_err("Variable name should be a word."));
        };

        Ok(Assign {
            target,
            value: Box::new(self.parse_node(current)?),
        })
    }

    fn build_if(&mut self) -> Result<If, CompilerError> {
        let mut current = next!(self);

        let test = self.parse_node(current)?;

        current = next!(self);

        if current.value != Type::LBrace {
            return Err(current.into_err("Expected a code block."));
        }

        let run = self.build_scope()?;

        current = peek!(self);
        let mut orelse = Node::None;
        if let Type::Keyword(kw) = current.value {
            if kw == Keyword::Else {
                next!(self);
                current = next!(self);
                orelse = self.parse_node(current)?;
            }
        }

        Ok(If {
            test: Box::new(test),
            run,
            orelse: Box::new(orelse),
        })
    }

    fn build_scope(&mut self) -> Result<Scope, CompilerError> {
        let mut current = next!(self);
        let mut body = vec![];

        if current.value == Type::LBrace {
            current = next!(self);
        }

        while current.value != Type::RBrace {
            if current.value != Type::Nl {
                body.push(self.parse_node(current)?);
            }
            current = next!(self);
        }

        Ok(Scope { body })
    }

    fn build_loop(&mut self) -> Result<Loop, CompilerError> {
        let body = self.build_scope()?;
        Ok(Loop { body })
    }

    fn build_return(&mut self) -> Result<Return, CompilerError> {
        let current = next!(self);

        let mut value = Node::None;

        if current.value != Type::Nl {
            value = self.parse_node(current)?;
        }

        Ok(Return {
            value: Box::new(value),
        })
    }

    fn build_binop(&mut self, start: Token) -> Result<BinOp, CompilerError> {
        let mut expr_unordered: Vec<ExprPart> = vec![];
        let mut need_closing = 0;
        let mut current = start;

        loop {
            let part = match current.value {
                Type::Op(op) => {
                    if expr_unordered.len() != 0 {
                        next!(self);
                    }
                    ExprPart::Operator(op)
                }

                Type::Primitive(_) => {
                    if expr_unordered.len() != 0 {
                        next!(self);
                    }
                    ExprPart::Operand(Node::Constant(self.build_constant(current)?))
                }

                Type::Word(_) => {
                    if expr_unordered.len() != 0 {
                        next!(self);
                    }
                    ExprPart::Operand(self.parse_node(current)?)
                }

                Type::LParen => {
                    if expr_unordered.len() != 0 {
                        next!(self);
                    }
                    need_closing += 1;
                    ExprPart::Lpar
                }
                Type::RParen => {
                    if need_closing == 0 {
                        break;
                    }
                    if expr_unordered.len() != 0 {
                        next!(self);
                    }
                    need_closing -= 1;
                    ExprPart::Rpar
                }
                Type::Nl => {
                    next!(self);
                    break;
                }
                Type::Comma | Type::RBrace => break,
                _ => panic!(),
            };

            expr_unordered.push(part);
            current = peek!(self);
        }

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

        Ok(
            match stack
                .pop()
                .expect("There should be at least something on the stack.")
            {
                Node::BinOp(binop) => binop,
                _ => panic!(),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap_in_main(case: &str) -> String {
        format!("fun main() {{ {} }}", case)
    }

    macro_rules! fun_def {
        ($name:tt, $args:expr, $body:expr, $returns:expr) => {
            Node::FunctionDef(FunctionDef {
                name: $name.to_owned(),
                args: $args,
                body: $body,
                returns: Box::new($returns),
            })
        };
        ($name:tt, $args:expr, $body:expr) => {
            fun_def!($name, $args, $body, Node::None)
        };
    }

    macro_rules! binop {
        ($left:expr, $op:expr, $right:expr) => {
            Node::BinOp(BinOp {
                left: Box::new($left),
                op: $op,
                right: Box::new($right),
            })
        };
    }

    macro_rules! constant {
        ($value:expr, i32) => {
            Node::Constant(Constant {
                value: Primitive::Int($value),
            })
        };
    }

    #[test]
    fn test_binop() {
        let test_case = "3 + 2 * 4";
        let mut parser = Parser::new();

        let ast = parser.parse(Lexer::from(&wrap_in_main(test_case))).unwrap();
        let expected = Module {
            body: vec![fun_def!(
                "main",
                vec![],
                vec![binop!(
                    constant!(3, i32),
                    Operator::Add,
                    binop!(constant!(2, i32), Operator::Mult, constant!(4, i32))
                ),]
            )],
        };

        assert_eq!(expected, ast);
    }
}
