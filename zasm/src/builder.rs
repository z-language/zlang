use std::{fs, io};

use crate::{
    constants::PUTS_SOURCE,
    func::Function,
    types::{Operator, Source, Store, StrPtr},
};

pub struct Builder {
    buffer: String,
    registers: Vec<Reg>,
    offset: u32,
}

pub struct Module<'guard> {
    globals: Vec<&'guard str>,
    strings: Vec<String>,
    functions: Vec<Function>,
}

impl<'guard> Module<'guard> {
    pub fn new() -> Self {
        Self {
            globals: vec!["_start"],
            strings: vec![],
            functions: vec![],
        }
    }

    pub fn add_func(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn add_global(&mut self, global: &'guard str) {
        self.globals.push(global);
    }

    pub fn add_string(&mut self, string: &str) -> StrPtr {
        self.strings.push(string.to_owned());
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

        out.push_str(PUTS_SOURCE);

        // section .data
        out.push_str("section .data\n");
        for (i, string) in self.strings.iter().enumerate() {
            let strn = format!(
                "str_{}: db \"{}\",0\n",
                i,
                string.replace('\n', "\", 0xA, \"")
            );
            out.push_str(&strn);
        }

        // section .bss
        out.push_str("section .bss\n");

        fs::write(file_name, out)
    }
}

#[derive(Debug)]
pub enum Operand {
    Reg(Reg),
    Int(i32),
    StrPtr(StrPtr),
    Var(Variable),
}

#[derive(Debug)]
pub struct Variable(u32);
#[derive(Debug)]
pub struct Reg(String);

impl Clone for Variable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Reg {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
    pub fn to_x64(&self) -> String {
        let mut out = String::new();
        if self.0.starts_with('a') {
            if let Some(first_char) = self.0.chars().next() {
                // Create a new string with the first character replaced
                out.push('a');
                out.push_str(&self.0[first_char.len_utf8()..]);
            }
        } else {
            out = self.0.clone();
            out.pop();
        }

        out
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            registers: vec![
                Reg::new("eax"),
                Reg::new("ecx"),
                Reg::new("edx"),
                Reg::new("ebx"),
                Reg::new("esi"),
                Reg::new("edi"),
                Reg::new("r8d"),
                Reg::new("r9d"),
                Reg::new("r10d"),
                Reg::new("r11d"),
                Reg::new("r12d"),
                Reg::new("r13d"),
                Reg::new("r14d"),
                Reg::new("r15d"),
            ],
            offset: 0,
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

    fn get_value(&mut self, value: Operand) -> String {
        match value {
            Operand::Reg(reg) => {
                let out = reg.0.clone();
                self.free_reg(reg);
                out
            }
            Operand::Int(i) => i.to_string(),
            Operand::Var(var) => {
                let reg = self.get_var(&var);
                let out = reg.0.clone();

                self.free_reg(reg);
                out
            }
            Operand::StrPtr(str) => str.to_string(),
        }
    }

    pub fn build_push(&mut self, value: Operand) {
        let value = match value {
            Operand::Reg(reg) => reg.to_x64(),
            Operand::Var(var) => {
                let reg = self.get_var(&var);
                let out = reg.to_x64();

                self.free_reg(reg);
                out
            }
            any => self.get_value(any),
        };
        let mut out = String::from("push ");
        out.push_str(&value);
        self.buffer.push_str(&self.format(&out));
    }

    pub fn assign_var(&mut self, value: Operand, var: &Variable) {
        let value = self.get_value(value);
        let out = format!("mov [rbp-{offset}], {value}", offset = var.0);
        self.buffer.push_str(&self.format(&out));
    }

    pub fn make_var(&mut self, value: Operand) -> Variable {
        self.offset += 4;
        let size = match value {
            Operand::Int(_) => "dword ",
            Operand::StrPtr(_) => "dword ",
            _ => "",
        };

        let value = self.get_value(value);

        let out = format!("mov {size}[rbp-{offset}], {value}", offset = self.offset);
        self.buffer.push_str(&self.format(&out));
        Variable(self.offset)
    }

    fn get_var(&mut self, var: &Variable) -> Reg {
        let reg = self.registers.pop().unwrap();

        let out = format!("mov {}, [rbp-{}]", reg.0, var.0);
        self.buffer.push_str(&self.format(&out));

        reg
    }

    pub fn build_op(&mut self, x: Operand, y: Operand, operation: Operator) -> Reg {
        let reg = match x {
            Operand::Reg(reg) => reg,
            Operand::Int(i) => {
                let reg = self.registers.pop().unwrap();

                let out = format!("mov {}, {}", reg.0, i);
                self.buffer.push_str(&self.format(&out));

                reg
            }
            Operand::Var(var) => self.get_var(&var),
            Operand::StrPtr(str) => {
                let reg = self.registers.pop().unwrap();

                let out = format!("mov {}, {}", reg.0, str.to_string());
                self.buffer.push_str(&self.format(&out));

                reg
            }
        };

        let source = self.get_value(y);

        let mut compare = false;
        let opcode = match operation {
            Operator::Add => "add",
            Operator::Sub => "sub",
            Operator::Mult => "mul",
            Operator::Div => "div",
            Operator::DoubleEquals
            | Operator::Greater
            | Operator::GreaterEquals
            | Operator::Less
            | Operator::LessEquals => {
                compare = true;
                "cmp"
            }
            Operator::Mod => todo!(),
        };

        let out = format!("{opcode} {register}, {source}", register = reg.0);
        self.buffer.push_str(&self.format(&out));

        if compare {
            let opcode = match operation {
                Operator::DoubleEquals => "sete",
                Operator::Greater => "setg",
                Operator::GreaterEquals => "setge",
                Operator::Less => "setl",
                Operator::LessEquals => "setle",
                _ => panic!(),
            };

            let mut out = String::from(opcode);
            out.push_str(" al");
            self.buffer.push_str(&self.format(&out));

            let out = format!("movzx {}, al", reg.0);
            self.buffer.push_str(&self.format(&out));
        }

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

    /// This function doesn't check is the called func
    /// exists, so make sure it does.
    pub fn call_by_name(&mut self, name: &str) {
        let out = format!("call {name}");
        self.buffer.push_str(&self.format(&out));
    }

    pub fn write_to_fn(&mut self, f: &mut Function) {
        f.write(&self.buffer);
        self.buffer.clear();
        self.offset = 0;
    }

    fn format(&self, value: &str) -> String {
        let mut out = String::new();

        out.push_str("    ");
        out.push_str(value);
        out.push('\n');

        out
    }
}
