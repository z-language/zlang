#[derive(Debug)]
pub enum Register64 {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

pub struct StrPtr(usize);

impl Source for StrPtr {}
impl ToString for StrPtr {
    fn to_string(&self) -> String {
        format!("str_{}", self.0.to_string())
    }
}
impl StrPtr {
    pub fn new(val: usize) -> Self {
        StrPtr(val)
    }
}

pub trait Store {}
pub trait Source {}

impl Store for Register64 {}
impl Source for Register64 {}
impl std::fmt::Display for Register64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format!("{:?}", self);
        write!(f, "{}", out.to_lowercase())
    }
}

impl Source for u8 {}
impl Source for u16 {}
impl Source for u32 {}
impl Source for u64 {}
impl Source for u128 {}
impl Source for usize {}
impl Source for i8 {}
impl Source for i16 {}
impl Source for i32 {}
impl Source for i64 {}
impl Source for i128 {}
impl Source for isize {}
