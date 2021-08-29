use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use reacto::span::{Span, S};
use reacto::chars::Chars;
use reacto::lex::Lex;
use reacto::ast::N;

use crate::ast::{
    Attr, Grammar, Ident, NamedRuleBody, Quantifier, Rule, RuleBody, RuleElement, RuleKind,
};
use crate::lexer::{LexError, Lexer};
use crate::token::Token;
use reacto::parse::{ParseCtx, Parse};

#[derive(Debug)]
pub struct ParseError {
    span: Span,
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
    UnexpectedToken(Token, Option<Token>),
    //(Vec<expected>, found?),
    UnexpectedTokenMulti(Vec<Token>, Option<Token>),
    // (sep, error)
    DuplicatedSepOrParseError(Token, Box<ParseError>),
    Eof,
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct Parser {
    ctx: ParseCtx<Token>,
}

fn remove_junk(tokens: &[S<Token>]) -> Vec<S<Token>> {
    let mut ret = vec![];
    for t in tokens {
        match t.tok {
            Token::Whitespace(_) => {}
            _ => ret.push(*t),
        }
    }
    ret
}

macro_rules! expect_one_of {
    ($parser:expr, $($l:path => $r:expr),*) => {{
        let d = $parser.expect_one_of(&[$($l),*])?;
        match d.kind {
            $($l => $r,)*
            _ => unreachable!(),
        }
    }}
}

macro_rules! parse_some {
    ($parser:expr, $f:ident, $sep:tt) => {{
        let head = $parser.$f()?;
        let mut ret = vec![head];
        while $parser.cmp_advance(T!$sep) {
            let d = $parser.$f()?;
            ret.push(d);
        }
        ret
    }}
}


impl Parser {
    pub fn new(chars: Chars, tokens: Vec<S<Token>>) -> Self {
        let tokens = remove_junk(&tokens);
        let ctx = ParseCtx::new(chars, tokens);
        Parser {ctx}
    }
}

impl Parser {
    pub fn parse_grammar(&mut self) -> Result<N<Grammar>> {
        self.parse_n(|parser| {
            let mut rules = vec![];
            while !parser.eof() {
                let rule = parser.parse_rule()?;
                rules.push(rule);
            }
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
            expect_one_of! { parser,
                Token::Alt => {
                    let rest =
                        parser.parse_some(|p| p.parse_named_rule_body(), Some(Token::Alt))?;
                    parser.expect(Token::Semicolon)?;
                    alts.extend(rest);
                },
                Token::Semicolon => {
                    // do nothing
                }
            };
            Ok(RuleKind::Enum(alts))
        }

        self.parse(|parser| {
            let attrs = parser.parse_many(|p| p.parse_attr(), None)?;
            let name = parser.parse_ident()?;
            parser.expect(Token::Colon)?;

            let kind = expect_one_of! { parser,
                Token::NumSign => {
                    let name = parser.parse_ident()?;
                    parse_alts(parser, name, None)?
                },
                Token::Ident => {
                    parser.back();
                    let body = parser.parse_rule_body()?;
                    if parser.cmp_advance(Token::NumSign) {
                        let name = parser.parse_ident()?;
                        parse_alts(parser, name, Some(body))?
                    } else {
                        parser.expect(Token::Semicolon)?;
                        RuleKind::Normal(body)
                    }
                }
            };
            Ok(Rule { attrs, name, kind })
        })
    }

    pub fn parse_named_rule_body(&mut self) -> Result<N<NamedRuleBody>> {
        self.parse(|parser| {
            let body = parser.parse_rule_body().ok();
            parser.expect(Token::NumSign)?;
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
            let (name, nt) = if parser.cmp_advance(Token::Assign) {
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
            let quantifier = expect_one_of! { parser,
                Token::Question => Quantifier::Maybe,
                Token::Asterisk => Quantifier::Multi,
                Token::Plus => Quantifier::AtLeastOne
            };

            Ok(quantifier)
        })
    }

    pub fn parse_ident(&mut self) -> Result<N<Ident>> {
        self.parse(|parser| {
            let d = parser.expect(Token::Ident)?;
            let name = parser.get_string(d.span);
            Ok(Ident { name })
        })
    }

    pub fn parse_attr(&mut self) -> Result<N<Attr>> {
        self.parse(|parser| {
            let d = parser.expect(Token::Attr)?;
            let name = parser.get_string(d.span);
            let name = name.trim_matches('@').to_string();
            Ok(Attr { name })
        })
    }
}

impl Parser {
    fn expect(&mut self, expected: Token) -> Result<Token> {
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

    fn expect_one_of(&mut self, expected: &[Token]) -> Result<Token> {
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

impl Parse  for Parser {
    type Error = ParseError;
    type Token = Token;

    fn ctx(&self) -> &ParseCtx<Self::Token> {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut ParseCtx<Self::Token> {
        &mut self.ctx_mut
    }

    fn eof_error(&mut self) -> Result<()> {
        let kind = ParseErrorKind::Eof;
        let span = self.span();
        Err(ParseError {kind, span})
    }
}

impl Parser {
    fn make_err<T>(&self, kind: ParseErrorKind) -> Result<T> {
        let span = self.current_span();
        Err(ParseError { span, kind })
    }
}
