mod common;

use common::read_file;
use zlang::{parser::core::Parser, tokenizer::Tokenizer};

#[test]
fn variables_run() {
    let mut tokenizer = Tokenizer::new();
    let mut parser = Parser::new();
    let source = read_file("examples/variables.ž");

    let tokens = tokenizer.tokenize(&source);
    let _ast = parser.parse(tokens);

    assert_eq!(1, 1);
}
