mod parser;
mod tokenizer;

use parser::core::Parser;
use std::env;
use std::fs;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let parser = Parser::new();
    let mut args = env::args();

    let filename = match args.nth(1) {
        Some(name) => name,
        None => {
            println!("Please specify a file name!");
            return;
        }
    };

    let source = match fs::read_to_string(filename.clone()) {
        Ok(source) => source,
        Err(_) => {
            println!("File: {} doesn't exist.", filename);
            return;
        }
    };

    let tokens = tokenizer.tokenize(source);

    let module = parser.parse(tokens);
    println!("{:?}", module);
}
