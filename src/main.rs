mod args;

use args::Args;
use clap::Parser;
use std::{fs, io::Write};
use zlang::compiler::Compiler as zCompiler;
use zlang::parser::Parser as zParser;
use zlang::tokenizer::Tokenizer as zLexer;

fn main() {
    let mut tokenizer = zLexer::new();
    let mut parser = zParser::new();
    let compiler = zCompiler::new();
    let args = Args::parse();

    let source = match fs::read_to_string(&args.file) {
        Ok(source) => source,
        Err(_) => {
            println!("File: {} doesn't exist.", args.file);
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

    if args.parse_only {
        println!("{:#?}", module);
        return;
    }

    let bytes = match compiler.compile(module) {
        Ok(prog) => prog,
        Err(err) => return println!("{:?}", err),
    };

    if args.dry_run {
        return;
    }
    let mut out = fs::File::create(args.out).unwrap();
    out.write_all(&bytes).unwrap();
}
