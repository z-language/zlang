use crate::tokenizer::*;
use Type::*;

pub fn get_tokenizer_variable_case() -> Vec<Token> {
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
            t_type: Op,
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
            t_type: Op,
        },
        Token {
            line: 2,
            pos: 18,
            value: "13".to_owned(),
            t_type: Number,
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
            t_type: Op,
        },
        Token {
            line: 4,
            pos: 14,
            value: "14".to_owned(),
            t_type: Number,
        },
    ]
}
