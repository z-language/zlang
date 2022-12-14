mod common;

use common::read_file;
use zlang::{parser::Parser, tokenizer::Tokenizer};

#[test]
fn function_run() {
    let mut tokenizer = Tokenizer::new();
    let mut parser = Parser::new();
    let source = read_file("examples/function.ž");

    let tokens = tokenizer.tokenize(&source).unwrap();
    let _ast = parser.parse(tokens).unwrap();
}
