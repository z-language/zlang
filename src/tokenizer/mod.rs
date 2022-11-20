mod tokenizer_tests;

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
                                self.throw(
                                    "String doesn't have a closing bracket...",
                                    Token {
                                        line: self.line,
                                        pos: self.pos,
                                        value: "a".to_owned(),
                                        t_type: Type::Nl,
                                    },
                                );
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
                '/' => tokens.push(Token {
                    line: self.line,
                    pos: self.pos,
                    value: "/".to_string(),
                    t_type: Type::Op,
                }),

                '-' => {
                    if self.get_offset((self.index + 1) as isize).unwrap_or('\r') == '>' {
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

    // TODO: improve
    fn highlight(&self, tok: Token) {
        for (i, line) in self.source.split("\n").enumerate() {
            let diff = tok.line.abs_diff(i as u32);

            if diff < 3 {
                println!("{}| {}", i + 1, line);
            }

            if diff == 0 {
                let spaces = " ".repeat(tok.pos as usize + 2);
                let arrows_num = match tok.t_type {
                    Type::Keyword => tok.value.len(),
                    Type::Word => tok.value.len(),
                    Type::Number => tok.value.len(),
                    Type::Op => tok.value.len(),

                    Type::Lparen => 1,
                    Type::Rparen => 1,
                    Type::Lbrack => 1,
                    Type::Rbrack => 1,

                    _ => 1,
                };
                println!("{}{}", spaces, "^".repeat(arrows_num));
            }
        }
    }

    fn throw(&self, message: &str, tok: Token) {
        eprintln!("Tokenizer error: {}", message);
        println!();

        self.highlight(tok);

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

fn is_keyword(word: &String) -> bool {
    if ["proc", "var", "mut", "return", "if", "elif", "else"].contains(&word.as_str()) {
        return true;
    }
    return false;
}
