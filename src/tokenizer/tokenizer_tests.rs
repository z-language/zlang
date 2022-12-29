use crate::tokenizer::{Token, Tokenizer, Type};

#[test]
fn test_strings_normal() {
    let mut tokenizer = Tokenizer::new();

    // Its important to note that here I'm escaping the rust's quotes, and not sending the escapes to the parser.
    let test_case = "\"test\"";
    let expected = Token {
        line: 1,
        pos: 0,
        value: "test".to_owned(),
        t_type: Type::String,
    };

    let got = tokenizer.tokenize(test_case).unwrap();
    let token = got.get(0).unwrap().clone();
    assert_eq!(got.len(), 1); // only one token expected
    assert_eq!(expected, token);
}

#[test]
fn test_string_escapes() {
    let mut tokenizer = Tokenizer::new();

    let test_case = "\"\\\"\\\"test\\\"\\\"\\\\\"";
    let expected = Token {
        line: 1,
        pos: 0,
        value: "\"\"test\"\"\\".to_owned(),
        t_type: Type::String,
    };

    let got = tokenizer.tokenize(test_case).unwrap();
    let token = got.get(0).unwrap().clone();
    assert_eq!(got.len(), 1); // only one token expected
    assert_eq!(expected, token);
}

#[test]
fn test_keywords() {
    let mut tokenizer = Tokenizer::new();

    let test_case = "int float var mut fun";
    let expected = vec![
        Token {
            line: 1,
            pos: 0,
            value: "int".to_owned(),
            t_type: Type::Keyword,
        },
        Token {
            line: 1,
            pos: 4,
            value: "float".to_owned(),
            t_type: Type::Keyword,
        },
        Token {
            line: 1,
            pos: 10,
            value: "var".to_owned(),
            t_type: Type::Keyword,
        },
        Token {
            line: 1,
            pos: 14,
            value: "mut".to_owned(),
            t_type: Type::Keyword,
        },
        Token {
            line: 1,
            pos: 18,
            value: "fun".to_owned(),
            t_type: Type::Keyword,
        },
    ];

    let got = tokenizer.tokenize(test_case).unwrap();

    assert_eq!(expected, got);
}

#[test]
fn test_variables() {
    let mut tokenizer = Tokenizer::new();

    let test_case = "var ime = \"luka\"
var mut starost = 13
var mut assignLater
assignLater = 14";

    let got = tokenizer.tokenize(test_case).unwrap();
    let expected = get_tokenizer_variable_case();
    assert_eq!(expected, got);
}

#[test]
fn test_math_expr_1() {
    let mut tokenizer = Tokenizer::new();
    let test_case = "2.5 * 3.75";
    let expected = vec![
        Token {
            line: 1,
            pos: 0,
            value: "2.5".to_owned(),
            t_type: Type::Float,
        },
        Token {
            line: 1,
            pos: 4,
            value: "*".to_owned(),
            t_type: Type::Op,
        },
        Token {
            line: 1,
            pos: 6,
            value: "3.75".to_owned(),
            t_type: Type::Float,
        },
    ];

    let tokens = tokenizer.tokenize(test_case).unwrap();

    assert_eq!(expected, tokens);
}

#[test]
fn test_math_expr_2() {
    let mut tokenizer = Tokenizer::new();
    let test_case = "(3 - 45) -";

    let tokens = tokenizer.tokenize(test_case).unwrap();
    let expected = vec![
        Token {
            line: 1,
            pos: 0,
            value: "(".to_owned(),
            t_type: Type::Lparen,
        },
        Token {
            line: 1,
            pos: 1,
            value: "3".to_owned(),
            t_type: Type::Int,
        },
        Token {
            line: 1,
            pos: 3,
            value: "-".to_owned(),
            t_type: Type::Op,
        },
        Token {
            line: 1,
            pos: 5,
            value: "45".to_owned(),
            t_type: Type::Int,
        },
        Token {
            line: 1,
            pos: 7,
            value: ")".to_owned(),
            t_type: Type::Rparen,
        },
        Token {
            line: 1,
            pos: 9,
            value: "-".to_owned(),
            t_type: Type::Op,
        },
    ];

    assert_eq!(expected, tokens);
}

#[test]
#[should_panic]
fn test_decimal_error() {
    let mut tokenizer = Tokenizer::new();
    let test_case = "5.34.7";

    tokenizer.tokenize(test_case).unwrap();
}

#[test]
fn test_num_formatting() {
    let mut tokenizer = Tokenizer::new();
    {
        let test_case = "1_000_000";
        let tokens = tokenizer.tokenize(test_case).unwrap();
        let expected = vec![Token {
            line: 1,
            pos: 0,
            value: "1000000".to_owned(),
            t_type: Type::Int,
        }];

        assert_eq!(expected, tokens);
    }

    {
        let test_case = "1___300_23400_";
        let tokens = tokenizer.tokenize(test_case).unwrap();
        let expected = vec![Token {
            line: 1,
            pos: 0,
            value: "130023400".to_owned(),
            t_type: Type::Int,
        }];

        assert_eq!(expected, tokens);
    }
}

#[test]
fn test_symbols() {
    let mut tokenizer = Tokenizer::new();
    let test_case = "-> -> :";

    let expected = vec![
        Token {
            line: 1,
            pos: 0,
            value: "->".to_owned(),
            t_type: Type::Arrow,
        },
        Token {
            line: 1,
            pos: 3,
            value: "->".to_owned(),
            t_type: Type::Arrow,
        },
        Token {
            line: 1,
            pos: 6,
            value: ":".to_owned(),
            t_type: Type::DoubleDot,
        },
    ];

    let tokens = tokenizer.tokenize(test_case).unwrap();

    assert_eq!(expected, tokens);
}

#[test]
fn test_comments() {
    let mut tokenizer = Tokenizer::new();
    let test_case = "// this is a comment";

    let tokens = tokenizer.tokenize(test_case).unwrap();

    assert!(tokens.is_empty());
}

#[test]
fn test_deq() {
    let mut tokenizer = Tokenizer::new();
    let test_case = "3 == 3";

    let expected = vec![
        Token {
            line: 1,
            pos: 0,
            value: "3".to_owned(),
            t_type: Type::Int,
        },
        Token {
            line: 1,
            pos: 2,
            value: "==".to_owned(),
            t_type: Type::Op,
        },
        Token {
            line: 1,
            pos: 5,
            value: "3".to_owned(),
            t_type: Type::Int,
        },
    ];

    let tokens = tokenizer.tokenize(test_case).unwrap();
    assert_eq!(expected, tokens);
}

#[test]
#[should_panic]
fn test_error() {
    let mut tokenizer = Tokenizer::new();
    let test_case = "\"unclosed string";

    let tokens = tokenizer.tokenize(test_case).unwrap();
    println!("{:?}", tokens);
}

pub fn get_tokenizer_variable_case() -> Vec<Token> {
    use super::Type::*;
    vec![
        Token {
            line: 1,
            pos: 0,
            value: "var".to_owned(),
            t_type: Keyword,
        },
        Token {
            line: 1,
            pos: 4,
            value: "ime".to_owned(),
            t_type: Word,
        },
        Token {
            line: 1,
            pos: 8,
            value: "=".to_owned(),
            t_type: Equals,
        },
        Token {
            line: 1,
            pos: 10,
            value: "luka".to_owned(),
            t_type: String,
        },
        Token {
            line: 1,
            pos: 16,
            value: "\n".to_owned(),
            t_type: Nl,
        },
        Token {
            line: 2,
            pos: 0,
            value: "var".to_owned(),
            t_type: Keyword,
        },
        Token {
            line: 2,
            pos: 4,
            value: "mut".to_owned(),
            t_type: Keyword,
        },
        Token {
            line: 2,
            pos: 8,
            value: "starost".to_owned(),
            t_type: Word,
        },
        Token {
            line: 2,
            pos: 16,
            value: "=".to_owned(),
            t_type: Equals,
        },
        Token {
            line: 2,
            pos: 18,
            value: "13".to_owned(),
            t_type: Int,
        },
        Token {
            line: 2,
            pos: 20,
            value: "\n".to_owned(),
            t_type: Nl,
        },
        Token {
            line: 3,
            pos: 0,
            value: "var".to_owned(),
            t_type: Keyword,
        },
        Token {
            line: 3,
            pos: 4,
            value: "mut".to_owned(),
            t_type: Keyword,
        },
        Token {
            line: 3,
            pos: 8,
            value: "assignLater".to_owned(),
            t_type: Word,
        },
        Token {
            line: 3,
            pos: 19,
            value: "\n".to_owned(),
            t_type: Nl,
        },
        Token {
            line: 4,
            pos: 0,
            value: "assignLater".to_owned(),
            t_type: Word,
        },
        Token {
            line: 4,
            pos: 12,
            value: "=".to_owned(),
            t_type: Equals,
        },
        Token {
            line: 4,
            pos: 14,
            value: "14".to_owned(),
            t_type: Int,
        },
    ]
}
