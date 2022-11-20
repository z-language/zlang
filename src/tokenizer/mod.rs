pub struct Tokenizer {
    source: String,
    line: u32,
    pos: u32,
    index: usize,
    len: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Keyword,
    Word,

    Lparen,
    Rparen,

    Nl,

    Lbrack,
    Rbrack,

    Op,
    Arrow,

    String,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: u32,
    pub pos: u32,
    pub value: String,
    pub t_type: Type,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            source: "".to_string(),
            line: 1,
            pos: 0,
            index: 0,
            len: 0,
        }
    }

    pub fn tokenize(&mut self, source: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        self.source = source;
        self.len = self.source.len();

        while self.index < self.len {
            let mut ch = self.source.chars().nth(self.index).unwrap();

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
                    let mut current = self.get_current();
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

                        if current != '\\' || self.get_offset(-1) == '\\' {
                            word.push(current);
                        }
                        len += 1;
                        self.incr();
                        current = self.get_current();
                    }
                    tokens.push(Token {
                        line: self.line,
                        pos: self.pos - len,
                        value: word,
                        t_type: Type::String,
                    });
                }

                ' ' => (),

                '+' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: "+".to_string(),
                    t_type: Type::Op,
                }),

                '=' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: "=".to_string(),
                    t_type: Type::Op,
                }),

                '*' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: "*".to_string(),
                    t_type: Type::Op,
                }),

                '-' => {
                    if self.get_offset((self.index + 1) as isize) == '>' {
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
                while ch.is_digit(10) {
                    content.push(ch);

                    self.incr();
                    if self.is_eof() {
                        break;
                    }
                    ch = self.get_current();
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
                    if self.is_eof() {
                        break;
                    }
                    ch = self.get_current();
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

    fn get_current(&self) -> char {
        self.source.chars().nth(self.index).unwrap()
    }

    fn get_offset(&self, offset: isize) -> char {
        self.source
            .chars()
            .nth(((self.index as isize) + offset) as usize)
            .unwrap()
    }

    fn is_eof(&self) -> bool {
        self.index >= self.len
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

fn is_keyword(word: &String) -> bool {
    if ["proc", "var", "mut", "return", "if", "elif", "else"].contains(&word.as_str()) {
        return true;
    }
    return false;
}
