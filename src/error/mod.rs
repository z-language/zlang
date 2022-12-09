use std::fmt::{self, Display};

const LINE_PADDING: u32 = 3;

#[derive(Debug, Clone)]
pub struct CompilerError<'guard> {
    line: u32,
    pos: u32,
    message: &'guard str,
}

impl<'guard> Display for CompilerError<'guard> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'guard> CompilerError<'guard> {
    pub fn new(line: u32, pos: u32, message: &'guard str) -> Self {
        CompilerError { line, pos, message }
    }

    pub fn display(&self, source: &str) {
        let mut line_num = self.line - LINE_PADDING;
        source
            .split('\n')
            .skip((self.line - LINE_PADDING) as usize)
            .take((LINE_PADDING * 2) as usize)
            .for_each(|line| {
                line_num += 1;
                if line_num == self.line + 1 {
                    let spaces = " ".repeat(self.pos as usize + 5);
                    let arrows = "^";
                    eprintln!("{}{} {}", spaces, arrows, self.message);
                }
                println!("{}| {}", padding(line_num), line)
            });

        println!();
    }
}

fn padding(num: u32) -> String {
    format!("{:<3}", num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding() {
        let got = padding(20);
        let expected = String::from("20 ");

        assert_eq!(expected, got);
    }
}
