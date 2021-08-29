use reacto::span::Span;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Token {
    Question,
    Plus,
    Asterisk,
    Colon,
    Semicolon,
    NumSign,
    Alt,
    Assign,
    Ident,
    Attr,
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
