pub const STDIN_FILENO: u32 = 0;
pub const STDOUT_FILENO: u32 = 1;

pub const PUTS_SOURCE: &str = "\
puts:
    push rbp
    mov rbp, rsp
    mov rax, 1
    mov rdi, 1
    mov rsi, [rsp+24]
    mov rdx, [rsp+16]
    syscall
    pop rbp
    ret
";
