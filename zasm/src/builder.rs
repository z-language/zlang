use std::{fs, io};

use crate::{
    func::Function,
    types::{Source, Store, StrPtr},
};

pub struct Reg(String);

pub struct Builder {
    buffer: String,
    registers: Vec<Reg>,
}

pub struct Module<'guard> {
    globals: Vec<&'guard str>,
    strings: Vec<&'guard str>,
    functions: Vec<Function>,
}

impl<'guard> Module<'guard> {
    pub fn new() -> Self {
        Self {
            globals: vec!["_start"],
            strings: vec![],
            functions: vec![Function::new("main")],
        }
    }

    pub fn add_func(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn get_main(&mut self) -> &mut Function {
        self.functions
            .first_mut()
            .expect("Main function always exists.")
    }

    pub fn add_global(&mut self, global: &'guard str) {
        self.globals.push(global);
    }

    pub fn add_string(&mut self, string: &'guard str) -> StrPtr {
        self.strings.push(string);
        StrPtr::new(self.strings.len() - 1)
    }

    pub fn write_to_file(&self, file_name: &str) -> Result<(), io::Error> {
        // globals
        let mut out = format!("global {}\n", self.globals.join(", "));

        // section .text
        out.push_str("section .text\n");
        out.push_str("_start:\n");
        out.push_str("    call main\n");

        out.push_str("    ; -- exit --\n");
        out.push_str("    mov rax, 60\n");
        out.push_str("    xor rdi, rdi\n");
        out.push_str("    syscall\n");

        for func in &self.functions {
            out.push_str(&func.to_string());
        }

        // section .data
        out.push_str("section .data\n");
        for (i, string) in self.strings.iter().enumerate() {
            let strn = format!("str_{}: db \"{}\",0xA\n", i, string);
            out.push_str(&strn);
        }

        // section .bss
        out.push_str("section .bss\n");

        fs::write(file_name, out)
    }
}

pub enum Operand {
    Reg(Reg),
    Int(i32),
}

impl Builder {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            registers: vec![
                Reg("rdx".to_owned()),
                Reg("rax".to_owned()),
                Reg("rdi".to_owned()),
            ],
        }
    }
    pub fn build_mov<T, E>(&mut self, dest: T, src: E) -> T
    where
        T: Store + ToString,
        E: Source + ToString,
    {
        let mut out = String::new();
        out.push_str("mov ");
        out.push_str(&dest.to_string());
        out.push_str(", ");
        out.push_str(&src.to_string());

        self.buffer.push_str(&self.format(&out));

        dest
    }

    pub fn build_add(&mut self, x: Operand, y: i32) -> Reg {
        let reg = match x {
            Operand::Reg(reg) => reg,
            Operand::Int(i) => {
                let reg = self.registers.pop().unwrap();

                let out = format!("mov {}, {}", reg.0, i);
                self.buffer.push_str(&self.format(&out));

                reg
            }
        };

        let out = format!("add {}, {}", reg.0, y);

        self.buffer.push_str(&self.format(&out));

        reg
    }

    pub fn free_reg(&mut self, x: Reg) {
        self.registers.push(x);
    }

    pub fn build_syscall(&mut self) {
        self.buffer.push_str(&self.format("syscall"));
    }

    pub fn build_call(&mut self, f: &Function) {
        let out = format!("call {}", f.name());
        self.buffer.push_str(&self.format(&out));
    }

    pub fn write_to_fn(&mut self, f: &mut Function) {
        f.write(&self.buffer);
        self.buffer.clear();
    }

    fn format(&self, value: &str) -> String {
        let mut out = String::new();

        out.push_str("    ");
        out.push_str(value);
        out.push('\n');

        out
    }
}
