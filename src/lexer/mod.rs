use std::{iter::Peekable, str::Chars};

use self::token::{SourcePos, Token, Type};
use crate::{error::CompilerError, grammar::*};

pub mod token;
// #[cfg(test)]
// mod tokenizer_tests;

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

macro_rules! tok_ok {
    ($self:ident, $x:expr) => {
        Ok(Token {
            pos: pos!($self),
            value: $x,
        })
    };
    ($pos:expr, $x:expr) => {
        Ok(token!($pos, $x))
    };
}

macro_rules! token {
    ($pos:expr, $x:expr) => {
        Token {
            pos: $pos,
            value: $x,
        }
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
            OPEN_PAREN => tok_ok!(self, Type::LParen),
            CLOSED_PAREN => tok_ok!(self, Type::RParen),
            OPEN_CBR => tok_ok!(self, Type::LBrace),
            CLOSED_CBR => tok_ok!(self, Type::Rbrace),
            COMMA => tok_ok!(self, Type::Comma),
            NL => {
                let ret = tok_ok!(self, Type::Nl);
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
                tok_ok!(pos!(column, self.line), Type::String(word))
            }
            COLON => tok_ok!(self, Type::DoubleDot),
            // Matches on arrow
            MINUS if *self.chars.peek()? == GREATER_THAN => {
                self.chars.next();
                self.column += 1;
                tok_ok!(pos!(self.column - 1, self.line), Type::Arrow)
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
                tok_ok!(pos!(self.column - 1, self.line), Type::Op("==".to_owned()))
            }

            // matches >=
            GREATER_THAN if *self.chars.peek()? == EQUALS => {
                self.column += 1;
                tok_ok!(pos!(self.column - 1, self.line), Type::Op(">=".to_owned()))
            }

            // matches <=
            LESS_THAN if *self.chars.peek()? == EQUALS => {
                self.column += 1;
                tok_ok!(pos!(self.column - 1, self.line), Type::Op("<=".to_owned()))
            }

            // TODO: better op parsing
            GREATER_THAN => tok_ok!(self, Type::Op(ch.to_string())),
            LESS_THAN => tok_ok!(self, Type::Op(ch.to_string())),
            MOD => tok_ok!(self, Type::Op(ch.to_string())),
            EQUALS => tok_ok!(self, Type::Equals),

            PLUS | STAR | FORWARD_SLASH | MINUS => tok_ok!(self, Type::Op(ch.to_string())),

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

                tok_ok!(
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

                tok_ok!(
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
    pub fn from(source: &'guard str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    fn throw(&self, message: &str) -> CompilerError {
        CompilerError::new(self.line as usize, self.column as usize, 1, message)
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

#[cfg(test)]
mod tests {
    use super::{token::SourcePos, token::Token, token::Type, Lexer};

    #[test]
    fn test_symbols() {
        let test_case = "(){},\n:";
        let mut lexer = Lexer::from(test_case);

        let expected: [Token; 7] = [
            token!(pos!(1, 1), Type::LParen),
            token!(pos!(2, 1), Type::RParen),
            token!(pos!(3, 1), Type::LBrace),
            token!(pos!(4, 1), Type::Rbrace),
            token!(pos!(5, 1), Type::Comma),
            token!(pos!(6, 1), Type::Nl),
            token!(pos!(1, 2), Type::DoubleDot),
        ];

        for token in &expected {
            assert_eq!(*token, lexer.next().unwrap().unwrap());
        }
    }
}
