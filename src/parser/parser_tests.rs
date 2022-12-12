use super::{ast::*, Parser};
use crate::tokenizer::{token::Token, Tokenizer};

#[test]
fn test_constant_expr() {
    let mut parser = Parser::new();
    let test_case = "3";

    let ast = parser.parse(get_tokens(test_case)).unwrap();
    let expected = Module {
        body: vec![Node::Constant(Constant {
            value: Primitive::Int(3),
        })],
    };

    assert_eq!(expected, ast);
}

#[test]
#[ignore = "not yet impl"]
fn test_binop() {
    let mut parser = Parser::new();
    let test_case = "3 + 5 * (6 -3)";

    let ast = parser.parse(get_tokens(test_case));
    println!("{:#?}", ast);
}

#[test]
fn test_fcall() {
    let mut parser = Parser::new();
    let test_case = "foo(1, mark, \"peter\", 6.7)";
    let expected = Module {
        body: vec![Node::Call(Call {
            func: Name {
                id: "foo".to_owned(),
            },
            args: vec![
                Node::Constant(Constant {
                    value: Primitive::Int(1),
                }),
                Node::Name(Name {
                    id: "mark".to_owned(),
                }),
                Node::Constant(Constant {
                    value: Primitive::Str("peter".to_owned()),
                }),
                Node::Constant(Constant {
                    value: Primitive::Float(6.7),
                }),
            ],
        })],
    };

    let ast = parser.parse(get_tokens(test_case)).unwrap();
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
    let ast = parser.parse(get_tokens(test_case)).unwrap();
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
    tokenizer.tokenize(test_case).unwrap()
}
