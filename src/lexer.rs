use std::error::Error;
use std::fmt;

use reacto::span::Span;
use reacto::chars::Chars;
use reacto::lex::Lex;

use crate::token::*;

#[derive(Debug)]
pub struct LexError {
    span: Span,
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
    LitStringNotClosed,
}

pub type Result<T> = std::result::Result<T, LexError>;

#[derive(Debug)]
pub struct Lexer {
    chars: Chars,
    cursor: usize,
    start: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars = Chars::new(input);
        Lexer {
            chars,
            cursor: 0,
            start: 0,
        }
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
            '=' => TokenKind::Assign,
            ' ' => TokenKind::Whitespace(Whitespace::Space),
            '\n' => TokenKind::Whitespace(Whitespace::Newline),
            '\r' => TokenKind::Whitespace(Whitespace::CarriageReturn),
            '\t' => TokenKind::Whitespace(Whitespace::HorizontalTab),
            '@' => {
                self.advance_while(is_digit_letter);
                TokenKind::Attr
            }
            '"' => {
                self.advance_while(|c| c != '"');
                if !self.advance_cmp('"') {
                    return Err(self.make_error(LexErrorKind::LitStringNotClosed));
                }
                TokenKind::LitString
            }
            c if is_letter(c) => {
                self.advance_while(is_digit_letter);
                TokenKind::Ident
            }
            c if is_digit(c) => return Err(self.make_error(LexErrorKind::NameStartWithDigit(c))),
            c => return Err(self.make_error(LexErrorKind::UnknownChar(c))),
        };
        Ok(Some(self.make_token(ty)))
    }

    fn make_token(&mut self, kind: TokenKind) -> Token {
        let span = self.span();
        let ret = Token {kind, span};
        self.start = self.cursor;
        ret
    }

    fn make_error(&mut self, kind: LexErrorKind) -> LexError {
        let span = self.span();
        LexError { span, kind }
    }
}

impl Lex for Lexer {
    fn chars(&self) -> &Chars {
        &self.chars
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn start(&self) -> usize {
        self.start
    }

    fn inc_cursor(&mut self) {
        self.cursor += 1
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
