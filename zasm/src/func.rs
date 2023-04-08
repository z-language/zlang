pub struct Function {
    name: String,
    text: String,
}
impl Function {
    pub fn new(name: &str) -> Self {
        Function {
            name: name.to_owned(),
            text: "".to_owned(),
        }
    }

    pub fn write(&mut self, text: &str) {
        self.text.push_str(text)
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        let mut out = String::from(self.name.to_owned());
        out.push_str(":\n");
        out.push_str("    push rbp\n");
        out.push_str("    mov rbp, rsp\n");
        out.push_str(&self.text);
        out.push_str("    pop rbp\n");
        out.push_str("    ret\n");
        out
    }
}
