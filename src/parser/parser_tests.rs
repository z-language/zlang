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
fn test_if_statements() {
    let mut parser = Parser::new();
    let test_case = "if cond {} else if cond2 {} else do()";
    let expected = Module {
        body: vec![Node::If(If {
            test: Box::new(Node::Name(Name {
                id: "cond".to_owned(),
            })),
            run: Box::new(Node::Scope(Scope { body: vec![] })),
            orelse: Box::new(Node::If(If {
                test: Box::new(Node::Name(Name {
                    id: "cond2".to_owned(),
                })),
                run: Box::new(Node::Scope(Scope { body: vec![] })),
                orelse: Box::new(Node::Call(Call {
                    func: Name {
                        id: "do".to_owned(),
                    },
                    args: vec![],
                })),
            })),
        })],
    };
    let ast = parser.parse(get_tokens(test_case)).unwrap();
    assert_eq!(expected, ast);
}

#[test]
fn test_binop_1() {
    let mut parser = Parser::new();
    let test_case = "3 + 5 * (6 -3)";
    let expected = Module {
        body: vec![Node::BinOp(BinOp {
            left: Box::new(Node::Constant(Constant {
                value: Primitive::Int(3),
            })),
            op: Operator::Add,
            right: Box::new(Node::BinOp(BinOp {
                left: Box::new(Node::Constant(Constant {
                    value: Primitive::Int(5),
                })),
                op: Operator::Mult,
                right: Box::new(Node::BinOp(BinOp {
                    left: Box::new(Node::Constant(Constant {
                        value: Primitive::Int(6),
                    })),
                    op: Operator::Sub,
                    right: Box::new(Node::Constant(Constant {
                        value: Primitive::Int(3),
                    })),
                })),
            })),
        })],
    };

    let ast = parser.parse(get_tokens(test_case)).unwrap();
    assert_eq!(expected, ast);
}

#[test]
fn test_return() {
    let mut parser = Parser::new();
    let test_case = "return 5";

    let expected = Module {
        body: vec![Node::Return(Return {
            value: Box::new(Node::Constant(Constant {
                value: Primitive::Int(5),
            })),
        })],
    };

    let ast = parser.parse(get_tokens(test_case)).unwrap();
    assert_eq!(expected, ast);
}

#[test]
fn test_binop_2() {
    let mut parser = Parser::new();
    let test_case = "foo() + 1\nage +2\n(3 - 2.5) * 4";
    let expected = Module {
        body: vec![
            Node::BinOp(BinOp {
                left: Box::new(Node::Call(Call {
                    func: Name {
                        id: "foo".to_owned(),
                    },
                    args: vec![],
                })),
                op: Operator::Add,
                right: Box::new(Node::Constant(Constant {
                    value: Primitive::Int(1),
                })),
            }),
            Node::BinOp(BinOp {
                left: Box::new(Node::Name(Name {
                    id: "age".to_owned(),
                })),
                op: Operator::Add,
                right: Box::new(Node::Constant(Constant {
                    value: Primitive::Int(2),
                })),
            }),
            Node::BinOp(BinOp {
                left: Box::new(Node::BinOp(BinOp {
                    left: Box::new(Node::Constant(Constant {
                        value: Primitive::Int(3),
                    })),
                    op: Operator::Sub,
                    right: Box::new(Node::Constant(Constant {
                        value: Primitive::Float(2.5),
                    })),
                })),
                op: Operator::Mult,
                right: Box::new(Node::Constant(Constant {
                    value: Primitive::Int(4),
                })),
            }),
        ],
    };

    let ast = parser.parse(get_tokens(test_case)).unwrap();
    assert_eq!(expected, ast);
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
fn test_scope() {
    let mut parser = Parser::new();
    let test_case = "{fun foo() {{}}{{}}}";
    let expected = Module {
        body: vec![Node::Scope(Scope {
            body: vec![
                Node::FunctionDef(FunctionDef {
                    name: "foo".to_owned(),
                    args: vec![],
                    body: vec![Node::Scope(Scope { body: vec![] })],
                    returns: Box::from(Node::None),
                }),
                Node::Scope(Scope {
                    body: vec![Node::Scope(Scope { body: vec![] })],
                }),
            ],
        })],
    };

    let ast = parser.parse(get_tokens(test_case)).unwrap();
    assert_eq!(expected, ast);
}

#[test]
fn test_loop() {
    let mut parser = Parser::new();
    let test_case = "loop {loop{}}";
    let expected = Module {
        body: vec![Node::Loop(Loop {
            body: vec![Node::Loop(Loop { body: vec![] })],
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
