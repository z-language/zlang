use crate::tokenizer::{Token, Tokenizer};

use super::{ast::*, core::Parser};

#[test]
fn test_constant_expr() {
    let mut parser = Parser::new();
    let test_case = String::from("3");

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

fn get_tokens(test_case: String) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(test_case)
}
