mod args;

use args::Args;
use clap::Parser;
use std::fs;
use z::compiler::Compiler as zCompiler;
use z::lexer::Lexer as zLexer;
use z::parser::Parser as zParser;

fn main() {
    let mut parser = zParser::new();
    let mut compiler = zCompiler::new();
    let args = Args::parse();

    let source = match fs::read_to_string(&args.file) {
        Ok(source) => source,
        Err(_) => {
            println!("File: {} doesn't exist.", args.file);
            return;
        }
    };
    let lexer = zLexer::from(&source);

    let module = match parser.parse(lexer) {
        Ok(module) => module,
        Err(err) => return err.display(&source),
    };

    if args.parse_only {
        println!("{:#?}", module);
        return;
    }

    match compiler.compile(module) {
        Ok(_) => (),
        Err(err) => return err.display(&source),
    }
}