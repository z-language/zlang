use crate::tokenizer::{token::Token, Tokenizer};

use super::{ast::*, core::Parser};

#[test]
#[ignore = "not yet implemented"]
fn test_constant_expr() {
    let mut parser = Parser::new();
    let test_case = "3";

    let ast = parser.parse(get_tokens(test_case));
    let expected = Module {
        body: vec![Node::Expr(Expr {
            value: Box::from(Node::Constant(Constant {
                value: Primitive::Int(3),
            })),
        })],
    };

    assert_eq!(expected, ast);
}

#[test]
#[ignore = "not yet implemented"]
fn test_func_def() {
    let mut parser = Parser::new();
    let test_case = "
    fun main() {
        fun foo() -> float {}

        fun foo2() {}
    }
    ";
    let ast = parser.parse(get_tokens(test_case));
    let expected = Module {
        body: vec![Node::FunctionDef(FunctionDef {
            name: todo!(),
            args: todo!(),
            body: todo!(),
            returns: todo!(),
        })],
    };
}

fn get_tokens(test_case: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(test_case)
}
