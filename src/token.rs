use crate::span::Span;

#[derive(Clone, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    Question,
    Plus,
    Asterisk,
    Colon,
    Semicolon,
    Name(String),
}
