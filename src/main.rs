mod parser;
mod tests;
mod tokenizer;

use parser::core::Parser;
use std::fs;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let parser = Parser::new();

    let source = fs::read_to_string("debug/variables.ž").unwrap();
    let tokens = tokenizer.tokenize(source);

    let module = parser.parse(tokens);
    println!("{:?}", module);
}
