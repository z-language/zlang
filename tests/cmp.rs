mod common;

use common::{read_file, run};
use zlang::{compiler::Compiler, lexer::Lexer, parser::Parser};

#[test]
fn cmp_run() {
    let mut tokenizer = Lexer::new();
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();
    let source = read_file("examples/cmp.ž");

    let tokens = tokenizer.tokenize(&source).unwrap();
    let ast = parser.parse(tokens).unwrap();
    let bytes = compiler.compile(ast).unwrap();

    run(bytes, "1\n0\n1\n1\n1\n0\n1\n0\n1\n0\n1\n").unwrap();
}
