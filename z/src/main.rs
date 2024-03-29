mod args;

use args::Args;
use clap::Parser;
use std::fs;
use std::process::Command;
use z::compiler::Compiler as zCompiler;
use z::lexer::Lexer as zLexer;
use z::parser::Parser as zParser;

const TEMPFILE: &str = "/tmp/.zcompiled";

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

    let module = match compiler.compile(module) {
        Ok(module) => module,
        Err(err) => return err.display(&source),
    };

    if args.dry_run {
        return;
    }

    let tmp_file = if let Some(file) = args.asm {
        if file.is_empty() {
            "out.asm".to_owned()
        } else {
            file
        }
    } else {
        TEMPFILE.to_owned()
    };

    module
        .write_to_file(&tmp_file)
        .expect("Failed to write to tempfile.");
    Command::new("nasm")
        .arg("-felf64")
        .arg("-g")
        .arg("-o")
        .arg(args.out)
        .arg(tmp_file)
        .spawn()
        .expect("Failed to run nasm.");
}
