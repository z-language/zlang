use std::borrow::Borrow;

use super::ast::{
    Arg, Assign, BinOp, Call, Constant, FunctionDef, If, Loop, Module, Name, Node, Operator,
    Primitive, Return, Scope, VariableDef,
};
use super::Parser;
use crate::error::{CompilerError, MakeErr};
use crate::grammar::*;
use crate::lexer::token::{Keyword, Token, Type};
use crate::lexer::Lexer;
use crate::parser::rpn::shutting_yard;

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
        }
    }

    pub fn parse(&mut self, lexer: Lexer<'guard>) -> Result<Module, CompilerError> {
        let mut module = Module::new();

        self.tokens = lexer.peekable();
        while self.tokens.peek().is_some() {
            let parsed = self.parse_node()?;
            module.body.push(parsed);
        }

        Ok(module)
    }
}

impl<'guard> Parser<'guard> {
    fn parse_node(&mut self) -> Result<Node, CompilerError> {
        let token = self.tokens.next().unwrap()?;
        match token.value {
            Type::Primitive(_) => Ok(Node::Constant(self.build_constant(token)?)),

            Type::Nl => return self.parse_node(),
            _ => {
                return Err(token
                    .clone()
                    .into_err(&*format!("Unexpected token: {:?}", token)))
            }
        }
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
}
