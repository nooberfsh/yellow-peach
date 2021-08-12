use std::error::Error;
use std::fmt;

use crate::token::{Token, TokenKind};
use crate::span::Span;
use crate::lexer::{Lexer, LexError, Chars};
use crate::ast::{Grammar, Rule, RuleBody, RuleElement, Quantifier};

#[derive(Debug)]
pub struct ParseError {
    span: Option<Span>,
    kind: ParseErrorKind,
}

impl fmt::Display for ParseError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

#[derive(Debug)]
enum ParseErrorKind {
    LexError(LexError)
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct Parser {
    chars: Chars,
    tokens: Vec<Token>,
    // state
    start: usize,
    cursor: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self> {
        let tokens = match lexer.tokens() {
            Ok(d) => d,
            Err(e) => return Err(ParseError{
                span: None,
                kind: ParseErrorKind::LexError(e),
            })
        };
        let chars = lexer.chars();
        Ok(Parser {
            chars,
            tokens,
            start:  0,
            cursor: 0,
        })
    }
}

impl Parser {
    pub fn parse_grammar(&mut self) -> Result<Grammar> {
        todo!()
    }

    pub fn parse_rule(&mut self) -> Result<Rule> {
        todo!()
    }

    pub fn parse_rule_body(&mut self) -> Result<RuleBody> {
        todo!()
    }

    pub fn parse_rule_element(&mut self) -> Result<RuleElement> {
        todo!()
    }

    pub fn parse_quantifier(&mut self) -> Result<Quantifier> {
        todo!()
    }
}
