mod common;

use common::read_file;
use zlang::{compiler::Compiler, lexer::Lexer, parser::Parser};

#[test]
fn loop_run() {
    let mut tokenizer = Lexer::new();
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();
    let source = read_file("examples/loop.ž");

    let tokens = tokenizer.tokenize(&source).unwrap();
    let ast = parser.parse(tokens).unwrap();

    // We can't run this code because it results in an endless loop.
    let _bytes = compiler.compile(ast).unwrap();
}
