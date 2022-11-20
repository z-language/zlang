mod tests;
mod tokenizer;

use std::fs;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new();

    let source = fs::read_to_string("debug/variables.ž").unwrap();
    let tokens = tokenizer.tokenize(source);

    println!("{:?}", tokens);
}
