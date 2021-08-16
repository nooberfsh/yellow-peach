use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::ast::{
    Grammar, Ident, NamedRuleBody, NodeId, Quantifier, Rule, RuleBody, RuleElement, RuleKind, N,
};
use crate::lexer::{Chars, LexError, Lexer};
use crate::span::Span;
use crate::token::{Token, TokenKind};

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
pub enum ParseErrorKind {
    LexError(LexError),
    //(expected, found?), None means eof
    UnexpectedToken(TokenKind, Option<Token>),
    //(Vec<expected>, found?),
    UnexpectedTokenMulti(Vec<TokenKind>, Option<Token>),
    // (sep, error)
    DuplicatedSepOrParseError(Token, Box<ParseError>),
    Eof,
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
struct IdGen {
    id: Arc<AtomicUsize>,
}

impl IdGen {
    fn new() -> Self {
        IdGen {
            id: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn next(&self) -> NodeId {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        NodeId(id)
    }
}

#[derive(Clone, Debug)]
pub struct Parser {
    chars: Chars,
    tokens: Vec<Token>,
    // state
    id_gen: IdGen,
    call_stack: Vec<usize>,
    cursor: usize,
}

fn remove_junk(tokens: &[Token]) -> Vec<Token> {
    let mut ret = vec![];
    for t in tokens {
        match t.kind {
            TokenKind::Whitespace(_) => {}
            _ => ret.push(*t),
        }
    }
    ret
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self> {
        let tokens = match lexer.tokens() {
            Ok(d) => d,
            Err(e) => {
                return Err(ParseError {
                    span: None,
                    kind: ParseErrorKind::LexError(e),
                })
            }
        };
        let tokens = remove_junk(&tokens);
        let chars = lexer.chars();
        Ok(Parser {
            chars,
            tokens,
            id_gen: IdGen::new(),
            call_stack: vec![],
            cursor: 0,
        })
    }
}

impl Parser {
    fn parse<T>(&mut self, f: impl Fn(&mut Parser) -> Result<T>) -> Result<N<T>> {
        let cursor = self.cursor;
        self.call_stack.push(self.cursor);

        if self.eof() {
            let e = self.make_err(ParseErrorKind::Eof);
            self.pop_stack();
            return e;
        }

        let ret = match f(self) {
            Ok(d) => Ok(self.make_node(d)),
            Err(e) => {
                self.cursor = cursor;
                Err(e)
            }
        };
        self.pop_stack();
        ret
    }

    // zero or more
    fn parse_many<T>(
        &mut self,
        f: impl Fn(&mut Parser) -> Result<T>,
        sep: Option<TokenKind>,
    ) -> Result<Vec<T>> {
        let mut ret = match f(self) {
            Ok(d) => vec![d],
            Err(_) => return Ok(vec![]),
        };

        if let Some(d) = sep {
            while self.cmp_advance(d) {
                match f(self) {
                    Ok(d) => ret.push(d),
                    Err(e) => {
                        self.back();
                        let sep = self.peek().unwrap();
                        return self
                            .make_err(ParseErrorKind::DuplicatedSepOrParseError(sep, Box::new(e)));
                    }
                }
            }
        } else {
            while let Ok(d) = f(self) {
                ret.push(d)
            }
        }

        Ok(ret)
    }

    // one or more
    fn parse_some<T>(
        &mut self,
        f: impl Fn(&mut Parser) -> Result<T>,
        sep: Option<TokenKind>,
    ) -> Result<Vec<T>> {
        let d = f(self)?;
        let mut ret = vec![d];
        let rest = self.parse_many(f, sep)?;
        ret.extend(rest);
        Ok(ret)
    }

    pub fn parse_grammar(&mut self) -> Result<N<Grammar>> {
        self.parse(|parser| {
            let rules = parser.parse_some(|p| p.parse_rule(), None)?;
            Ok(Grammar { rules })
        })
    }

    pub fn parse_rule(&mut self) -> Result<N<Rule>> {
        fn parse_alts(
            parser: &mut Parser,
            name: N<Ident>,
            body: Option<N<RuleBody>>,
        ) -> Result<RuleKind> {
            let head = parser.make_node(NamedRuleBody { name, body });
            let mut alts = vec![head];
            if parser.cmp_advance(TokenKind::Alt) {
                let rest =
                    parser.parse_some(|p| p.parse_named_rule_body(), Some(TokenKind::Alt))?;
                parser.expect(TokenKind::Semicolon)?;
                alts.extend(rest);
            } else {
                parser.expect(TokenKind::Semicolon)?;
            }
            Ok(RuleKind::Enum(alts))
        }

        self.parse(|parser| {
            let name = parser.parse_ident()?;
            parser.expect(TokenKind::Colon)?;

            let token = parser.expect_one_of(&[TokenKind::NumSign, TokenKind::Ident])?;
            let kind = match token.kind {
                TokenKind::NumSign => {
                    let name = parser.parse_ident()?;
                    parse_alts(parser, name, None)?
                }
                TokenKind::Ident => {
                    parser.back();
                    let body = parser.parse_rule_body()?;
                    if parser.cmp_advance(TokenKind::NumSign) {
                        let name = parser.parse_ident()?;
                        parser.expect(TokenKind::Alt)?;
                        parse_alts(parser, name, Some(body))?
                    } else {
                        parser.expect(TokenKind::Semicolon)?;
                        RuleKind::Normal(body)
                    }
                }
                _ => unreachable!(),
            };
            Ok(Rule { name, kind })
        })
    }

    pub fn parse_named_rule_body(&mut self) -> Result<N<NamedRuleBody>> {
        self.parse(|parser| {
            let body = parser.parse_rule_body().ok();
            parser.expect(TokenKind::NumSign)?;
            let name = parser.parse_ident()?;
            Ok(NamedRuleBody { name, body })
        })
    }

    pub fn parse_rule_body(&mut self) -> Result<N<RuleBody>> {
        self.parse(|parser| {
            let body = parser.parse_some(|p| p.parse_rule_element(), None)?;
            Ok(RuleBody { body })
        })
    }

    pub fn parse_rule_element(&mut self) -> Result<N<RuleElement>> {
        self.parse(|parser| {
            let name = parser.parse_ident()?;
            let (name, nt) = if parser.cmp_advance(TokenKind::Assign) {
                let nt = parser.parse_ident()?;
                (Some(name), nt)
            } else {
                (None, name)
            };
            let quantifier = parser.parse_quantifier().ok();
            Ok(RuleElement {
                name,
                nt,
                quantifier,
            })
        })
    }

    pub fn parse_quantifier(&mut self) -> Result<N<Quantifier>> {
        self.parse(|parser| {
            let d = parser.expect_one_of(&[
                TokenKind::Question,
                TokenKind::Asterisk,
                TokenKind::Plus,
            ])?;
            let quantifier = match d.kind {
                TokenKind::Question => Quantifier::Maybe,
                TokenKind::Asterisk => Quantifier::Multi,
                TokenKind::Plus => Quantifier::AtLeastOne,
                _ => unreachable!(),
            };
            Ok(quantifier)
        })
    }

    pub fn parse_ident(&mut self) -> Result<N<Ident>> {
        self.parse(|parser| {
            let d = parser.expect(TokenKind::Ident)?;
            let name = parser.get_string(d.span);
            Ok(Ident { name })
        })
    }
}

impl Parser {
    fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        let d = match self.advance() {
            Some(d) => d,
            None => return self.make_err(ParseErrorKind::UnexpectedToken(expected, None)),
        };
        if d.kind == expected {
            Ok(d)
        } else {
            let kind = ParseErrorKind::UnexpectedToken(expected, Some(d));
            self.make_err(kind)
        }
    }

    fn expect_one_of(&mut self, expected: &[TokenKind]) -> Result<Token> {
        let d = match self.advance() {
            Some(d) => d,
            None => {
                return self.make_err(ParseErrorKind::UnexpectedTokenMulti(
                    expected.to_vec(),
                    None,
                ))
            }
        };

        if expected.iter().find(|e| *e == &d.kind).is_some() {
            Ok(d)
        } else {
            self.make_err(ParseErrorKind::UnexpectedTokenMulti(
                expected.to_vec(),
                Some(d),
            ))
        }
    }

    fn get_string(&self, span: Span) -> String {
        let s = &self.chars[span.start()..span.end()];
        s.iter().collect()
    }
}

impl Parser {
    fn assert_call_stack(&self) {
        assert!(!self.call_stack.is_empty(), "not int call stack")
    }

    fn pop_stack(&mut self) -> Option<usize> {
        self.call_stack.pop()
    }

    fn eof(&self) -> bool {
        self.cursor == self.tokens.len()
    }

    fn peek(&self) -> Option<Token> {
        if self.eof() {
            None
        } else {
            Some(self.tokens[self.cursor])
        }
    }

    fn back(&mut self) {
        self.cursor -= 1;
    }

    fn advance_if(&mut self, p: impl Fn(TokenKind) -> bool) -> bool {
        if let Some(c) = self.peek() {
            if p(c.kind) {
                self.cursor += 1;
                return true;
            }
        }
        false
    }

    fn cmp_advance(&mut self, ty: TokenKind) -> bool {
        self.advance_if(|x| x == ty)
    }

    fn advance(&mut self) -> Option<Token> {
        if self.eof() {
            None
        } else {
            let c = self.tokens[self.cursor];
            self.cursor += 1;
            Some(c)
        }
    }

    fn current_span(&self) -> Option<Span> {
        if let Some(start) = self.call_stack.last() {
            if start < &self.cursor {
                let start = self.tokens[*start].span;
                let end = self.tokens[self.cursor - 1].span;
                Some(start.merge(end))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn make_node<T>(&self, t: T) -> N<T> {
        self.assert_call_stack();
        assert!(self.call_stack.last().unwrap() < &self.cursor);

        let id = self.id_gen.next();
        let span = self.current_span().unwrap();
        let ret = N { id, span, t };
        ret
    }

    fn make_err<T>(&self, kind: ParseErrorKind) -> Result<T> {
        let span = self.current_span();
        Err(ParseError { span, kind })
    }
}
