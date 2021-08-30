use std::error::Error;
use std::fmt;

use reacto::lex::{Lex, LexCtx};
use reacto::span::Span;

use crate::token::*;

#[derive(Debug)]
pub struct LexError {
    span: Span,
    kind: LexErrorKind,
}

#[derive(Debug)]
enum LexErrorKind {
    NameStartWithDigit(char),
    UnknownChar(char),
    LitStringNotClosed,
}

pub type Result<T> = std::result::Result<T, LexError>;

#[derive(Clone, Debug)]
pub struct Lexer {
    ctx: LexCtx,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let ctx = LexCtx::new(input);
        Lexer { ctx }
    }

    fn make_error(&mut self, kind: LexErrorKind) -> LexError {
        let span = self.span();
        LexError { span, kind }
    }
}

impl Lex for Lexer {
    type Token = Token;
    type Error = LexError;

    fn ctx(&self) -> &LexCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut LexCtx {
        &mut self.ctx
    }

    fn next(&mut self) -> Result<Option<Token>> {
        let c = match self.advance() {
            Some(d) => d,
            None => return Ok(None),
        };

        let ty = match c {
            '?' => Token::Question,
            '+' => Token::Plus,
            '*' => Token::Asterisk,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '#' => Token::NumSign,
            '|' => Token::Alt,
            '=' => Token::Assign,
            ' ' => Token::Whitespace(Whitespace::Space),
            '\n' => Token::Whitespace(Whitespace::Newline),
            '\r' => Token::Whitespace(Whitespace::CarriageReturn),
            '\t' => Token::Whitespace(Whitespace::HorizontalTab),
            '@' => {
                self.advance_while(is_digit_letter);
                Token::Attr
            }
            '"' => {
                self.advance_while(|c| c != '"');
                if !self.advance_cmp('"') {
                    return Err(self.make_error(LexErrorKind::LitStringNotClosed));
                }
                Token::LitString
            }
            c if is_letter(c) => {
                self.advance_while(is_digit_letter);
                Token::Ident
            }
            c if is_digit(c) => return Err(self.make_error(LexErrorKind::NameStartWithDigit(c))),
            c => return Err(self.make_error(LexErrorKind::UnknownChar(c))),
        };
        Ok(Some(ty))
    }
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
