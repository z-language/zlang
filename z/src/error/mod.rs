use std::fmt;

const LINE_PADDING: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerError {
    line: usize,
    pos: usize,
    arrows: usize,
    message: String,
}

impl fmt::Display for CompilerError {
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
        let spaces = " ".repeat(self.pos + 4);
        let arrows = "^".repeat(self.arrows);
        eprintln!("{}{} {}", spaces, arrows, self.message);
    }

    pub fn display(&self, source: &str) {
        let mut displayed = false;
        let mut line_num = 0;
        if LINE_PADDING < self.line {
            line_num = self.line - LINE_PADDING;
        }

        source
            .split('\n')
            .skip(line_num)
            .take(LINE_PADDING * 2)
            .for_each(|line| {
                if line_num == self.line {
                    self.print_message();
                    displayed = true;
                }
                line_num += 1;
                println!("{}| {}", padding(line_num), line)
            });

        if !displayed {
            self.print_message();
        }

        println!();
    }
}

fn padding(num: usize) -> String {
    format!("{:<3}", num)
}

pub trait MakeErr {
    fn into_err(self, message: &str) -> CompilerError;
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
