use super::{ast::*, core::Parser};
use crate::tokenizer::{token::Token, Tokenizer};

#[test]
fn test_constant_expr() {
    let mut parser = Parser::new();
    let test_case = "3";

    let ast = parser.parse(get_tokens(test_case));
    let expected = Module {
        body: vec![Node::Constant(Constant {
            value: Primitive::Int(3),
        })],
    };

    assert_eq!(expected, ast);
}

#[test]
fn test_func_def() {
    let mut parser = Parser::new();
    let test_case = "
    fun main() {
        fun foo(x: int, y: float) -> float {}

        fun foo2() {}
    }
    ";
    let ast = parser.parse(get_tokens(test_case));
    let expected = Module {
        body: vec![Node::FunctionDef(FunctionDef {
            name: "main".to_owned(),
            args: vec![],
            body: vec![
                Node::FunctionDef(FunctionDef {
                    name: "foo".to_owned(),
                    args: vec![
                        Node::Arg(Arg {
                            name: "x".to_owned(),
                            annotation: Box::from(Node::Name(Name {
                                id: "int".to_owned(),
                            })),
                        }),
                        Node::Arg(Arg {
                            name: "y".to_owned(),
                            annotation: Box::from(Node::Name(Name {
                                id: "float".to_owned(),
                            })),
                        }),
                    ],
                    body: vec![],
                    returns: Box::from(Node::Name(Name {
                        id: "float".to_owned(),
                    })),
                }),
                Node::FunctionDef(FunctionDef {
                    name: "foo2".to_owned(),
                    args: vec![],
                    body: vec![],
                    returns: Box::from(Node::None),
                }),
            ],
            returns: Box::from(Node::None),
        })],
    };

    assert_eq!(expected, ast);
}

fn get_tokens(test_case: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new();
    tokenizer.tokenize(test_case)
}
