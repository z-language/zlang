use std::{iter::Peekable, str::Chars};

use self::token::{Keyword, SourcePos, Token, Type};
use crate::{
    error::CompilerError,
    grammar::*,
    parser::{ast::Primitive, ZResult},
};
use zasm::types::Operator;

pub mod token;
// #[cfg(test)]
// mod tokenizer_tests;

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        SourcePos::new($x, $y)
    };
    ($self:ident) => {
        SourcePos::new($self.column, $self.line)
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
    type Item = ZResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.column += 1;
        let ch = self.chars.next()?;
        let tok = match ch {
            OPEN_PAREN => tok_ok!(self, Type::LParen),
            CLOSED_PAREN => tok_ok!(self, Type::RParen),
            OPEN_CBR => tok_ok!(self, Type::LBrace),
            CLOSED_CBR => tok_ok!(self, Type::RBrace),
            COMMA => tok_ok!(self, Type::Comma),
            OPEN_BR => tok_ok!(self, Type::LBracket),
            CLOSED_BR => tok_ok!(self, Type::RBracket),
            NL | SEMICOLON => {
                let ret = tok_ok!(self, Type::Nl);
                self.line += 1;
                self.column = 0;

                return Some(ret);
            }
            SPACE => return self.next(),
            DOUBLE_QUOTES => {
                let mut word = String::new();
                let column = self.column;

                let mut current = self.chars.next()?;
                while current != DOUBLE_QUOTES {
                    if current == BACKSLASH {
                        current = match self.chars.next()? {
                            'n' => '\n',
                            DOUBLE_QUOTES => DOUBLE_QUOTES,
                            BACKSLASH => BACKSLASH,
                            _ => return Some(Err(self.throw("Unknown escape char."))),
                        };
                    }

                    word.push(current);
                    current = self.chars.next()?;
                }

                self.column += word.len() as u32 + 1;
                tok_ok!(
                    pos!(column, self.line),
                    Type::Primitive(Primitive::Str(word))
                )
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
                self.chars.next();
                self.column += 1;
                tok_ok!(
                    pos!(self.column - 1, self.line),
                    Type::Op(Operator::DoubleEquals)
                )
            }

            // matches >=
            GREATER_THAN if *self.chars.peek()? == EQUALS => {
                self.chars.next();
                self.column += 1;
                tok_ok!(
                    pos!(self.column - 1, self.line),
                    Type::Op(Operator::GreaterEquals)
                )
            }

            // matches <=
            LESS_THAN if *self.chars.peek()? == EQUALS => {
                self.chars.next();
                self.column += 1;
                tok_ok!(
                    pos!(self.column - 1, self.line),
                    Type::Op(Operator::LessEquals)
                )
            }

            GREATER_THAN => tok_ok!(self, Type::Op(Operator::Greater)),
            LESS_THAN => tok_ok!(self, Type::Op(Operator::Less)),
            MOD => tok_ok!(self, Type::Op(Operator::Mod)),
            PLUS => tok_ok!(self, Type::Op(Operator::Add)),
            STAR => tok_ok!(self, Type::Op(Operator::Mult)),
            FORWARD_SLASH => tok_ok!(self, Type::Op(Operator::Div)),
            EQUALS => tok_ok!(self, Type::Equals),

            case if case.is_numeric() || (case == MINUS && self.chars.peek()?.is_numeric()) => {
                let mut content = String::new();
                if case == MINUS {
                    content.push('-');
                    content.push(self.chars.next()?);
                } else {
                    content.push(case);
                }

                let mut floating = false;
                let column = self.column;

                while let Some(current) = self.chars.peek() {
                    if *current == DOT {
                        floating = true;
                    } else if *current == UNDERSCORE {
                        self.chars.next();
                        continue;
                    } else if !current.is_numeric() {
                        break;
                    }

                    content.push(self.chars.next()?);
                }
                self.column += content.len() as u32 - 1;

                tok_ok!(
                    pos!(column, self.line),
                    if floating {
                        Type::Primitive(Primitive::Float(content.parse().unwrap()))
                    } else {
                        Type::Primitive(Primitive::Int(content.parse().unwrap()))
                    }
                )
            }
            MINUS => tok_ok!(self, Type::Op(Operator::Sub)),

            case if case.is_alphabetic() || case == '_' => {
                let mut word = String::from(case);
                let column = self.column;

                while let Some(current) = self.chars.peek() {
                    if !current.is_alphanumeric() && *current != '_' {
                        break;
                    }

                    word.push(self.chars.next().expect("We already peeked this value."));
                }
                self.column += word.len() as u32 - 1;

                tok_ok!(
                    pos!(column, self.line),
                    if is_keyword(&word) {
                        Type::Keyword(match_keyword(&word))
                    } else {
                        Type::Word(word)
                    }
                )
            }

            EXCLAMATION if *self.chars.peek()? == EQUALS => {
                self.chars.next().expect("Shouldn't fail");
                tok_ok!(self, Type::Op(Operator::NotEquals))
            }

            EXCLAMATION => tok_ok!(self, Type::Not),

            _ => return Some(Err(self.throw("Unexpected char."))),
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
}

fn is_keyword(word: &str) -> bool {
    [FUN, VAR, MUT, RETURN, IF, ELSE, TRUE, FALSE, LOOP, BREAK].contains(&word)
}

fn match_keyword(word: &str) -> Keyword {
    match word {
        TRUE => Keyword::True,
        FALSE => Keyword::True,
        FUN => Keyword::Fun,
        VAR => Keyword::Var,
        MUT => Keyword::Mut,
        IF => Keyword::If,
        ELSE => Keyword::Else,
        BREAK => Keyword::Break,
        LOOP => Keyword::Loop,
        RETURN => Keyword::Return,
        _ => panic!("Keyword: '{}' isn't implemented yet.", word),
    }
}

impl<'guard> Default for Lexer<'guard> {
    fn default() -> Self {
        Self {
            chars: "".chars().peekable(),
            line: 0,
            column: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::Primitive;

    use super::{token::SourcePos, token::Token, token::Type, Lexer};

    #[test]
    fn test_symbols() {
        let test_case = "(){},\n:";
        let mut lexer = Lexer::from(test_case);

        let expected: [Token; 7] = [
            token!(pos!(1, 1), Type::LParen),
            token!(pos!(2, 1), Type::RParen),
            token!(pos!(3, 1), Type::LBrace),
            token!(pos!(4, 1), Type::RBrace),
            token!(pos!(5, 1), Type::Comma),
            token!(pos!(6, 1), Type::Nl),
            token!(pos!(1, 2), Type::DoubleDot),
        ];

        for token in expected {
            assert_eq!(token, lexer.next().unwrap().unwrap());
        }
    }

    #[test]
    fn test_numbers() {
        let test_case = "23 2.5 1_349__2_";
        let mut lexer = Lexer::from(test_case);

        let expected = [
            token!(pos!(1, 1), Type::Primitive(Primitive::Int(23))),
            token!(pos!(4, 1), Type::Primitive(Primitive::Float(2.5))),
            token!(pos!(8, 1), Type::Primitive(Primitive::Int(13492))),
        ];

        for token in expected {
            assert_eq!(token, lexer.next().unwrap().unwrap());
        }
    }
}
