mod common;

use common::{read_file, run};
use zlang::{compiler::Compiler, lexer::Lexer, parser::Parser};

#[test]
fn flow_run() {
    let mut tokenizer = Lexer::new();
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();
    let source = read_file("examples/flow.ž");

    let tokens = tokenizer.tokenize(&source).unwrap();
    let ast = parser.parse(tokens).unwrap();
    let bytes = compiler.compile(ast).unwrap();

    run(bytes, "yes\nmaybe\ndoing else\n5\n").unwrap();
}
