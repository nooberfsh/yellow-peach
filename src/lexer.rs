use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::span::Span;
use crate::token::*;
use std::ops::Deref;

#[derive(Debug)]
pub struct LexError {
    span: Option<Span>,
    kind: LexErrorKind,
}

impl fmt::Display for LexError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for LexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

#[derive(Debug)]
enum LexErrorKind {
    NameStartWithDigit(char),
    UnknownChar(char),
}

pub type Result<T> = std::result::Result<T, LexError>;

#[derive(Debug)]
pub struct Lexer {
    chars: Chars,
    cursor: usize,
    start: usize,
}

#[derive(Debug, Clone)]
pub struct Chars(Arc<Vec<char>>);

impl Deref for Chars {
    type Target = [char];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<_> = input.chars().collect();
        Lexer {
            chars: Chars(Arc::new(chars)),
            cursor: 0,
            start: 0,
        }
    }

    pub fn chars(&self) -> Chars {
        self.chars.clone()
    }

    pub fn tokens(&mut self) -> Result<Vec<Token>> {
        let mut ret = vec![];
        while let Some(t) = self.next()? {
            ret.push(t);
        }
        Ok(ret)
    }

    pub fn next(&mut self) -> Result<Option<Token>> {
        let c = match self.advance() {
            Some(d) => d,
            None => return Ok(None),
        };

        let ty = match c {
            '?' => TokenKind::Question,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Asterisk,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '#' => TokenKind::NumSign,
            '|' => TokenKind::Alt,
            c if is_letter(c) => {
                self.advance_while(is_digit_letter);
                TokenKind::Ident
            }
            c if is_digit(c) => return Err(self.make_error(LexErrorKind::NameStartWithDigit(c))),
            c => return Err(self.make_error(LexErrorKind::UnknownChar(c))),
        };
        Ok(Some(self.make_token(ty)))
    }

    fn eof(&self) -> bool {
        self.cursor == self.chars.len()
    }

    fn advance_while(&mut self, p: impl Fn(char) -> bool) -> usize {
        let mut num = 0;
        while let Some(c) = self.peek() {
            if !p(c) {
                break;
            }
            self.cursor += 1;
            num += 1;
        }
        num
    }

    fn advance(&mut self) -> Option<char> {
        if self.eof() {
            None
        } else {
            let c = self.chars[self.cursor];
            self.cursor += 1;
            Some(c)
        }
    }

    fn peek(&self) -> Option<char> {
        if self.eof() {
            None
        } else {
            Some(self.chars[self.cursor])
        }
    }

    fn make_token(&mut self, k: TokenKind) -> Token {
        let ret = Token {
            kind: k,
            span: Span::new(self.start, self.cursor),
        };
        self.start = self.cursor;
        ret
    }

    fn make_error(&mut self, kind: LexErrorKind) -> LexError {
        let span = if self.cursor == self.start {
            None
        } else {
            Some(Span::new(self.start, self.cursor))
        };
        LexError { span, kind }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// helper functions

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_letter(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
}

fn is_digit_letter(c: char) -> bool {
    is_digit(c) || is_letter(c)
}
