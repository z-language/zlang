use std::{iter::Peekable, str::Chars};

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
            column: $self.column,
        }
    };
}

macro_rules! tok {
    ($self:ident, $x:expr) => {
        Ok(Token {
            pos: pos!($self),
            value: $x,
        })
    };
    ($pos:expr, $x:expr) => {
        Ok(Token {
            pos: $pos,
            value: $x,
        })
    };
}

pub struct Lexer<'guard> {
    chars: Peekable<Chars<'guard>>,
    line: u32,
    column: u32,
}

impl<'guard> Iterator for Lexer<'guard> {
    type Item = Result<Token, CompilerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.column += 1;
        let ch = self.chars.next()?;
        let tok: Result<Token, CompilerError> = match ch {
            OPEN_PAREN => tok!(self, Type::LParen),
            CLOSED_PAREN => tok!(self, Type::RParen),
            OPEN_CBR => tok!(self, Type::LBrace),
            CLOSED_CBR => tok!(self, Type::Rbrace),
            COMMA => tok!(self, Type::Comma),
            NL => {
                let ret = tok!(self, Type::Nl);
                self.line += 1;
                self.column = 0;

                return Some(ret);
            }
            SPACE => return self.next(),
            DOUBLE_QUOTES => {
                // TODO: make this better
                let mut word = String::new();
                let column = self.column;

                let mut current = self.chars.next()?;
                while current != '"' {
                    word.push(current);

                    current = self.chars.next()?;
                }

                self.column += word.len() as u32 + 1;
                tok!(pos!(column, self.line), Type::String(word))
            }
            COLON => tok!(self, Type::DoubleDot),
            // Matches on arrow
            MINUS if *self.chars.peek()? == GREATER_THAN => {
                self.chars.next();
                self.column += 1;
                tok!(pos!(self.column - 1, self.line), Type::Arrow)
            }
            FORWARD_SLASH if *self.chars.peek()? == FORWARD_SLASH => {
                while *self.chars.peek()? != NL {
                    self.chars.next();
                }
                self.column -= 1;

                return self.next();
            }

            // matches ==
            EQUALS if *self.chars.peek()? == EQUALS => {
                self.column += 1;
                tok!(pos!(self.column - 1, self.line), Type::Op("==".to_owned()))
            }

            // matches >=
            GREATER_THAN if *self.chars.peek()? == EQUALS => {
                self.column += 1;
                tok!(pos!(self.column - 1, self.line), Type::Op(">=".to_owned()))
            }

            // matches <=
            LESS_THAN if *self.chars.peek()? == EQUALS => {
                self.column += 1;
                tok!(pos!(self.column - 1, self.line), Type::Op("<=".to_owned()))
            }

            // TODO: better op parsing
            GREATER_THAN => tok!(self, Type::Op(ch.to_string())),
            LESS_THAN => tok!(self, Type::Op(ch.to_string())),
            MOD => tok!(self, Type::Op(ch.to_string())),
            EQUALS => tok!(self, Type::Equals),

            PLUS | STAR | FORWARD_SLASH | MINUS => tok!(self, Type::Op(ch.to_string())),

            case if case.is_numeric() => {
                // TODO: preparse numbers
                let mut content = String::from(case);
                let mut floating = false;
                let column = self.column;

                while let Some(current) = self.chars.peek() {
                    if *current == DOT {
                        floating = true;
                    } else if !current.is_numeric() {
                        break;
                    }

                    content.push(self.chars.next()?);
                }
                self.column += content.len() as u32 - 1;

                tok!(
                    pos!(column, self.line),
                    if floating {
                        Type::Float(content)
                    } else {
                        Type::Int(content)
                    }
                )
            }

            case if case.is_alphabetic() => {
                let mut word = String::from(case);
                let column = self.column;

                while let Some(current) = self.chars.peek() {
                    if !current.is_alphanumeric() {
                        break;
                    }

                    word.push(self.chars.next().expect("We already peeked this value."));
                }
                self.column += word.len() as u32 - 1;

                tok!(
                    pos!(column, self.line),
                    if is_keyword(&word) {
                        Type::Keyword(word)
                    } else {
                        Type::Word(word)
                    }
                )
            }

            _ => return Some(Err(self.throw("Unexpeced char."))),
        };

        Some(tok)
    }
}

impl<'guard> Lexer<'guard> {
    pub fn new() -> Self {
        Lexer {
            chars: todo!(),
            line: todo!(),
            column: todo!(),
        }
    }

    pub fn from(source: &'guard str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    fn throw(&self, message: &str) -> CompilerError {
        CompilerError::new(1, 1, 1, message)
    }

    #[deprecated]
    pub fn tokenize(&mut self, source: &str) -> Result<Vec<Token>, CompilerError> {
        Ok(vec![])
    }
}

fn is_keyword(word: &str) -> bool {
    [
        FUN, VAR, MUT, RETURN, IF, ELSE, INT, FLOAT, TRUE, FALSE, LOOP, BREAK,
    ]
    .contains(&word)
}
