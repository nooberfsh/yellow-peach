use std::error::Error;
use std::fmt;

use iterable::Iterable;
use reacto::span::{Span, S};
use reacto::lex::Lex;
use reacto::ast::N;
use reacto::*;
use reacto::parse::{ParseCtx, Parse};

use crate::ast::{
    Attr, Grammar, Ident, NamedRuleBody, Quantifier, Rule, RuleBody, RuleElement, RuleKind,
};
use crate::lexer::{LexError, Lexer};
use crate::token::Token;

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
    UnexpectedToken(Token, Option<S<Token>>),
    //(Vec<expected>, found?),
    UnexpectedTokenMulti(Vec<Token>, Option<S<Token>>),
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


impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self>  {
        let tokens = match lexer.tokens() {
            Ok(d) => d,
            Err(e) => {
                let span = None;
                let kind = ParseErrorKind::LexError(e);
                return Err(ParseError{span ,kind})
            }
        };
        let chars = lexer.chars();
        let tokens = remove_junk(&tokens);
        let ctx = ParseCtx::new(chars.clone(), tokens);
        Ok(Parser {ctx})
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
                    let rest = parse_some!(parser, parse_named_rule_body, Token::Alt);
                    parser.expect(Token::Semicolon)?;
                    alts.extend(rest);
                },
                Token::Semicolon => {
                    // do nothing
                }
            };
            Ok(RuleKind::Enum(alts))
        }

        self.parse_n(|parser| {
            let attrs = parse_many_l1!(parser, parse_attr, Token::Attr);
            let name = parser.parse_ident()?;
            parser.expect(Token::Colon)?;

            let kind = sat_one_of! { parser,
                Token::NumSign => {
                    parser.advance();
                    let name = parser.parse_ident()?;
                    parse_alts(parser, name, None)?
                },
                Token::Ident => {
                    let body = parser.parse_rule_body()?;
                    if parser.advance_cmp(Token::NumSign) {
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
        self.parse_n(|parser| {
            let body = parser.parse_rule_body().ok();
            parser.expect(Token::NumSign)?;
            let name = parser.parse_ident()?;
            Ok(NamedRuleBody { name, body })
        })
    }

    pub fn parse_rule_body(&mut self) -> Result<N<RuleBody>> {
        self.parse_n(|parser| {
            let body = parse_some_l1!(parser, parse_rule_element, Token::Ident);
            Ok(RuleBody { body })
        })
    }

    pub fn parse_rule_element(&mut self) -> Result<N<RuleElement>> {
        self.parse_n(|parser| {
            let name = parser.parse_ident()?;
            let (name, nt) = if parser.advance_cmp(Token::Assign) {
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
        self.parse_n(|parser| {
            let quantifier = expect_one_of! { parser,
                Token::Question => Quantifier::Maybe,
                Token::Asterisk => Quantifier::Multi,
                Token::Plus => Quantifier::AtLeastOne
            };

            Ok(quantifier)
        })
    }

    pub fn parse_ident(&mut self) -> Result<N<Ident>> {
        self.parse_n(|parser| {
            let d = parser.expect(Token::Ident)?;
            let name = parser.get_string(d.span);
            Ok(Ident { name })
        })
    }

    pub fn parse_attr(&mut self) -> Result<N<Attr>> {
        self.parse_n(|parser| {
            let d = parser.expect(Token::Attr)?;
            let name = parser.get_string(d.span);
            let name = name.trim_matches('@').to_string();
            Ok(Attr { name })
        })
    }
}

impl Parse  for Parser {
    type Error = ParseError;
    type Token = Token;

    fn ctx(&self) -> &ParseCtx<Self::Token> {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut ParseCtx<Self::Token> {
        &mut self.ctx
    }

    fn expect_err(&self, expected: Self::Token, found: Option<S<Self::Token>>) -> Self::Error {
        let span = if let Some(d) = &found {
            Some(d.span)
        } else {
            None
        };
        let kind = ParseErrorKind::UnexpectedToken(expected, found);
        ParseError{span ,kind}
    }

    fn expect_one_of_err(
        &self,
        expected: &[Self::Token],
        found: Option<S<Self::Token>>,
    ) -> Self::Error {
        let span = if let Some(d) = &found {
            Some(d.span)
        } else {
            None
        };
        let kind = ParseErrorKind::UnexpectedTokenMulti(expected.map(|t|t.clone()), found);
        ParseError{span ,kind}
    }
}
