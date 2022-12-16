use std::fmt::{self, Display};

const LINE_PADDING: usize = 10;

#[derive(Debug, Clone)]
pub struct CompilerError {
    line: usize,
    pos: usize,
    arrows: usize,
    message: String,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CompilerError {
    pub fn new(line: usize, pos: usize, len: usize, message: &str) -> Self {
        let arrows = if len == 0 { 1 } else { len };

        CompilerError {
            line,
            pos,
            arrows,
            message: message.to_owned(),
        }
    }

    fn print_message(&self) {
        let spaces = " ".repeat(self.pos + 5);
        let arrows = "^".repeat(self.arrows);
        eprintln!("{}{} {}", spaces, arrows, self.message);
    }

    pub fn display(&self, source: &str) {
        let mut line_num = 0;
        if LINE_PADDING < self.line {
            line_num = self.line - LINE_PADDING;
        }

        source
            .split('\n')
            .skip(line_num)
            .take(LINE_PADDING * 2)
            .for_each(|line| {
                line_num += 1;
                if line_num == self.line + 1 {
                    self.print_message();
                }
                println!("{}| {}", padding(line_num), line)
            });

        if line_num == 1 {
            self.print_message();
        }

        println!();
    }
}

fn padding(num: usize) -> String {
    format!("{:<3}", num)
}

pub trait MakeErr {
    fn into_err(&self, message: &str) -> CompilerError;
    fn into_err_offset(&self, offset: i32, message: &str) -> CompilerError;
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
