use self::token::{Token, Type};

pub mod token;
#[cfg(test)]
mod tokenizer_tests;

pub struct Tokenizer<'a> {
    source: &'a str,
    line: u32,
    pos: u32,
    index: usize,
    len: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new() -> Self {
        Tokenizer {
            source: "",
            line: 1,
            pos: 0,
            index: 0,
            len: 0,
        }
    }

    pub fn tokenize(&mut self, source: &'a str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        self.source = source;
        self.len = self.source.len();

        while self.index < self.len {
            let mut ch = self
                .get_current()
                .expect("Something went horribly wrong...");

            match ch {
                '(' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: "(".to_string(),
                    t_type: Type::Lparen,
                }),
                ')' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: ")".to_string(),
                    t_type: Type::Rparen,
                }),
                '{' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: "{".to_string(),
                    t_type: Type::Lbrack,
                }),
                '}' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: "}".to_string(),
                    t_type: Type::Rbrack,
                }),
                '\n' => {
                    tokens.push(Token {
                        line: self.line,
                        pos: self.pos,
                        value: "\n".to_string(),
                        t_type: Type::Nl,
                    });
                    self.line += 1;
                    self.pos = 0;
                    self.index += 1;
                    continue;
                }
                '"' => {
                    self.incr();
                    let mut current = self
                        .get_current()
                        .expect("Souldn't fail because it just gets the char we matched on.");
                    let mut escaped = false;
                    let mut word = String::new();
                    let mut len = 1;

                    loop {
                        if current == '"' && !escaped {
                            break;
                        }

                        if current == '\\' && !escaped {
                            escaped = true;
                        } else {
                            escaped = false;
                        }

                        if current != '\\'
                            || self
                                .get_offset(-1)
                                .expect("It gets the previous char which definetly exists.")
                                == '\\'
                        {
                            word.push(current);
                        }
                        len += 1;
                        self.incr();
                        current = match self.get_current() {
                            Some(chr) => chr,
                            None => {
                                self.throw("String doesn't have a closing bracket.");
                                '0' // here just so that the compiler doesn't complain
                            }
                        }
                    }
                    tokens.push(Token {
                        line: self.line,
                        pos: self.pos - len,
                        value: word,
                        t_type: Type::String,
                    });
                }

                ' ' => (),

                ':' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: ":".to_owned(),
                    t_type: Type::Diacritic,
                }),

                '+' | '=' | '*' | '/' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: String::from(ch),
                    t_type: Type::Op,
                }),

                '-' => {
                    if self.get_offset(1).unwrap_or('\r') == '>' {
                        tokens.push(Token {
                            line: self.line,
                            pos: self.pos,
                            value: "->".to_string(),
                            t_type: Type::Arrow,
                        });
                        self.incr();
                        continue;
                    }

                    tokens.push(Token {
                        line: self.line,
                        pos: self.pos,
                        value: "-".to_string(),
                        t_type: Type::Op,
                    });
                }

                _ => (),
            }

            if ch.is_digit(10) {
                let mut content = String::new();
                let pos = self.pos;
                let mut had_point = false;
                while ch.is_digit(10) || ch == '.' {
                    if ch == '.' {
                        if had_point {
                            self.throw("A number can only have one decimal point.");
                        }
                        had_point = true;
                    }
                    content.push(ch);

                    self.incr();

                    if let Some(chr) = self.get_current() {
                        ch = chr;
                    } else {
                        break;
                    }
                }

                self.decr();

                tokens.push(Token {
                    line: self.line,
                    pos,
                    value: content,
                    t_type: Type::Number,
                })
            }

            if ch.is_alphabetic() {
                let mut content = String::new();
                let pos = self.pos;
                while ch.is_alphanumeric() {
                    content.push(ch);

                    self.incr();

                    if let Some(chr) = self.get_current() {
                        ch = chr;
                    } else {
                        break;
                    }
                }

                self.decr();

                tokens.push(Token {
                    line: self.line,
                    pos,
                    t_type: if is_keyword(&content) {
                        Type::Keyword
                    } else {
                        Type::Word
                    },
                    value: content,
                })
            }

            self.incr();
        }

        tokens
    }

    fn get_current(&self) -> Option<char> {
        self.get_nth(self.index)
    }

    fn get_offset(&self, offset: isize) -> Option<char> {
        self.get_nth(((self.index as isize) + offset) as usize)
    }

    fn get_nth(&self, n: usize) -> Option<char> {
        self.source.chars().nth(n)
    }

    fn throw(&self, message: &str) {
        eprintln!("Tokenizer error: {}", message);
        println!();

        panic!()
    }

    fn incr(&mut self) {
        self.pos += 1;
        self.index += 1;
    }

    fn decr(&mut self) {
        self.pos -= 1;
        self.index -= 1;
    }
}

fn is_keyword(word: &str) -> bool {
    if [
        "fun", "var", "mut", "return", "if", "elif", "else", "int", "float",
    ]
    .contains(&word)
    {
        return true;
    }
    return false;
}
