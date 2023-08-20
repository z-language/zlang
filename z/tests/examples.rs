use std::{fs, process::Command, time::Duration};

use z::{compiler::Compiler, error::CompilerError, lexer::Lexer, parser::Parser};

const EXAMPLES_PATH: &str = "./examples";
const ASM_FILE: &str = "../build/out.asm";
const OBJECT_FILE: &str = "../build/out.o";
const EXE_FILE: &str = "../build/a.out";

#[test]
fn test_examples() -> Result<(), CompilerError> {
    let files = fs::read_dir(EXAMPLES_PATH).expect("Failed to list examples dir.");
    let mut compiler = Compiler::new();

    for file in files {
        let path = file.unwrap().path();
        let source = fs::read_to_string(path).expect("Failed to read file.");

        let lexer = Lexer::from(&source);
        let mut parser = Parser::new();

        let ast = parser.parse(lexer)?;
        let module = compiler.compile(ast)?;

        module
            .write_to_file(ASM_FILE)
            .expect("Failed to write to file.");

        Command::new("nasm")
            .arg("-felf64")
            .arg("-g")
            .arg("-o")
            .arg(OBJECT_FILE)
            .arg(ASM_FILE)
            .spawn()
            .unwrap()
            .wait()
            .expect("Failed to run nasm.");

        Command::new("ld")
            .arg("-o")
            .arg(EXE_FILE)
            .arg(OBJECT_FILE)
            .spawn()
            .unwrap()
            .wait()
            .expect("Failed to link executable");

        let mut handle = Command::new(EXE_FILE)
            .spawn()
            .expect("Failed to run compiled executable.");

        std::thread::sleep(Duration::from_millis(10));
        handle.kill().expect("Failed to kill subprocess.");
    }

    Ok(())
}
