pub struct Function {
    name: String,
    text: String,
    reserved: u32,
}
impl Function {
    pub fn new(name: &str) -> Self {
        Function {
            name: name.to_owned(),
            text: "".to_owned(),
            reserved: 0,
        }
    }

    pub fn write(&mut self, text: &str) {
        self.text.push_str(text)
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn set_reserved(&mut self, x: u32) {
        self.reserved = x;
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        let mut out = String::from(self.name.to_owned());
        out.push_str(":\n");
        out.push_str("    push rbp\n");
        out.push_str("    mov rbp, rsp\n");

        if self.reserved > 0 {
            out.push_str("    sub rsp, ");
            out.push_str(&self.reserved.to_string());
            out.push('\n');
        }

        out.push_str(&self.text);
        out.push_str("    leave\n");
        out.push_str("    ret\n");
        out
    }
}
