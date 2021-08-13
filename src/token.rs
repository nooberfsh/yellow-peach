use crate::span::Span;

#[derive(Clone, Debug, Copy)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum TokenKind {
    Question,
    Plus,
    Asterisk,
    Colon,
    Semicolon,
    Ident,
}
