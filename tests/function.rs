mod common;

use common::{read_file, run};
use zlang::{compiler::Compiler, lexer::Lexer, parser::Parser};

#[test]
#[ignore = "yikes"]
fn function_run() {
    let source = read_file("examples/function.ž");
    let lexer = Lexer::from(&source);
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();

    let ast = parser.parse(lexer).unwrap();
    let bytes = compiler.compile(ast).unwrap();

    run(bytes, "").unwrap();
}
