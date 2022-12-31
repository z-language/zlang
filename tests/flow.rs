mod common;

use common::{read_file, run};
use zlang::{compiler::Compiler, lexer::Lexer, parser::Parser};

#[test]
#[ignore = "yikes"]
fn flow_run() {
    let source = read_file("examples/flow.ž");
    let lexer = Lexer::from(&source);
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();

    let ast = parser.parse(lexer).unwrap();
    let bytes = compiler.compile(ast).unwrap();

    run(bytes, "yes\nmaybe\ndoing else\n5\n").unwrap();
}
