use self::token::{SourcePos, Token, Type};
use crate::{error::CompilerError, grammar::*};

pub mod token;
#[cfg(test)]
mod tokenizer_tests;

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        SourcePos {
            line: $y,
            column: $x,
        }
    };
    ($self:ident) => {
        SourcePos {
            line: $self.line,
            column: $self.pos,
        }
    };
}

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

    pub fn tokenize(&mut self, source: &'a str) -> Result<Vec<Token>, CompilerError> {
        let mut tokens: Vec<Token> = vec![];

        self.source = source;
        self.line = 1;
        self.pos = 0;
        self.index = 0;
        self.len = self.source.len();

        while self.index < self.len {
            let mut ch = self
                .get_current()
                .expect("Something went horribly wrong...");

            match ch {
                OPEN_PAREN => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Lparen,
                }),
                CLOSED_PAREN => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Rparen,
                }),
                OPEN_CBR => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Lbrack,
                }),
                CLOSED_CBR => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Rbrack,
                }),
                COMMA => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Comma,
                }),
                NL => {
                    tokens.push(Token {
                        pos: pos!(self),
                        value: Type::Nl,
                    });
                    self.line += 1;
                    self.pos = 0;
                    self.index += 1;
                    continue;
                }
                DOUBLE_QUOTES => {
                    self.incr();
                    let mut current = self
                        .get_current()
                        .expect("Souldn't fail because it just gets the char we matched on.");
                    let mut escaped = false;
                    let mut word = String::new();
                    let mut len = 1;

                    loop {
                        if current == DOUBLE_QUOTES && !escaped {
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
                                return Err(self.throw("String doesn't have a closing bracket."))
                            }
                        }
                    }
                    tokens.push(Token {
                        pos: pos!(self.pos - len, self.line),
                        value: Type::String(word),
                    });
                }

                SPACE => (),

                COLON => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::DoubleDot,
                }),

                // Matches on arrow
                MINUS if self.get_offset(1).unwrap_or('0') == GREATER_THAN => {
                    tokens.push(Token {
                        pos: pos!(self),
                        value: Type::Arrow,
                    });
                    self.incr();
                    self.incr();
                }

                // Matches on a comment
                FORWARD_SLASH if self.get_offset(1).unwrap_or('0') == FORWARD_SLASH => {
                    let mut current = ch;
                    while current != NL {
                        self.incr();
                        current = match self.get_current() {
                            Some(chr) => chr,
                            None => break,
                        }
                    }
                    self.decr();
                }

                EQUALS if self.get_offset(1).unwrap_or('0') == EQUALS => {
                    tokens.push(Token {
                        pos: pos!(self),
                        value: Type::Op("==".to_owned()),
                    });
                    self.incr();
                }

                GREATER_THAN if self.get_offset(1).unwrap_or('0') == EQUALS => {
                    tokens.push(Token {
                        pos: pos!(self),
                        value: Type::Op(">=".to_owned()),
                    });
                    self.incr();
                }

                LESS_THAN if self.get_offset(1).unwrap_or('0') == EQUALS => {
                    tokens.push(Token {
                        pos: pos!(self),
                        value: Type::Op("<=".to_owned()),
                    });
                    self.incr();
                }

                GREATER_THAN => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Op(ch.to_string()),
                }),

                LESS_THAN => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Op(ch.to_string()),
                }),

                MOD => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Op(ch.to_string()),
                }),

                EQUALS => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Equals,
                }),

                PLUS | STAR | FORWARD_SLASH | MINUS => tokens.push(Token {
                    pos: pos!(self),
                    value: Type::Op(ch.to_string()),
                }),

                case if case.is_numeric() => {
                    let mut content = String::new();
                    let pos = self.pos;
                    let mut had_point = false;
                    while ch.is_numeric() || ch == DOT || ch == UNDERSCORE {
                        if ch == DOT {
                            if had_point {
                                return Err(self.throw("A number can only have one decimal point."));
                            }
                            had_point = true;
                        }

                        if ch != UNDERSCORE {
                            content.push(ch);
                        }

                        self.incr();

                        if let Some(chr) = self.get_current() {
                            ch = chr;
                        } else {
                            break;
                        }
                    }

                    self.decr();

                    tokens.push(Token {
                        pos: pos!(pos, self.line),
                        value: if had_point {
                            Type::Float(content)
                        } else {
                            Type::Int(content)
                        },
                    })
                }

                case if case.is_alphabetic() => {
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
                        pos: pos!(pos, self.line),
                        value: if is_keyword(&content) {
                            Type::Keyword(content)
                        } else {
                            Type::Word(content)
                        },
                    })
                }

                _ => return Err(self.throw("Unexpeced char.")),
            }

            self.incr();
        }

        Ok(tokens)
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

    fn throw(&self, message: &'a str) -> CompilerError {
        CompilerError::new(self.line as usize, self.pos as usize, 1, message)
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
    [
        FUN, VAR, MUT, RETURN, IF, ELSE, INT, FLOAT, TRUE, FALSE, LOOP, BREAK,
    ]
    .contains(&word)
}
