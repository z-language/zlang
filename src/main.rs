use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use zlang::compiler::Compiler;
use zlang::parser::Parser;
use zlang::tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new();
    let mut parser = Parser::new();
    let compiler = Compiler::new();
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

    let tokens = match tokenizer.tokenize(&source) {
        Ok(toks) => toks,
        Err(err) => return err.display(&source),
    };

    let module = match parser.parse(tokens) {
        Ok(module) => module,
        Err(err) => return err.display(&source),
    };

    let bytes = match compiler.compile(module) {
        Ok(prog) => prog,
        Err(err) => return println!("{:?}", err),
    };

    let mut out = File::create("debug/main").unwrap();
    out.write_all(&bytes).unwrap();
}
