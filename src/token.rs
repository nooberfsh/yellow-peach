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
    NumSign,
    Alt,
    Assign,
    Ident,
    Whitespace(Whitespace),
    LitString,
}

// https://www.unf.edu/~cwinton/html/cop3601/s10/class.notes/C4-PurgeBlnkLns.pdf
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Whitespace {
    Space,
    Newline,
    CarriageReturn,
    HorizontalTab,
}
