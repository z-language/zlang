mod args;

use args::Args;
use clap::Parser;
use std::fs;
use zlang::compiler::Compiler as zCompiler;
use zlang::lexer::Lexer as zLexer;
use zlang::parser::Parser as zParser;

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

    println!("{:#?}", module);

    /*
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
     */
}
