mod common;

use common::{read_file, run};
use zlang::{compiler::Compiler, parser::Parser, tokenizer::Tokenizer};

#[test]
fn variables_run() {
    let mut tokenizer = Tokenizer::new();
    let mut parser = Parser::new();
    let mut compiler = Compiler::new();
    let source = read_file("examples/variables.ž");

    let tokens = tokenizer.tokenize(&source).unwrap();
    let ast = parser.parse(tokens).unwrap();
    let bytes = compiler.compile(ast).unwrap();

    run(bytes, "hello to \"mark\" C:\\Drive\\neki\n").unwrap();
}
