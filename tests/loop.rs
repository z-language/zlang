mod common;

use common::read_file;
use zlang::{compiler::Compiler, lexer::Lexer, parser::Parser};

#[test]
fn loop_run() {
    let source = read_file("examples/loop.ž");
    let lexer = Lexer::from(&source);
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();

    let ast = parser.parse(lexer).unwrap();

    // We can't run this code because it results in an endless loop.
    let _bytes = compiler.compile(ast).unwrap();
}
