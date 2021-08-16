use std::fmt;

use crate::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct NodeId(pub(crate) usize);

#[derive(Clone)]
pub struct N<T> {
    pub span: Span,
    pub id: NodeId,
    pub t: T,
}

impl<T: fmt::Debug> fmt::Debug for N<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.t.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: Vec<N<Rule>>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: N<Ident>,
    pub kind: RuleKind,
}

#[derive(Debug, Clone)]
pub enum RuleKind {
    Enum(Vec<N<NamedRuleBody>>),
    Normal(N<RuleBody>),
    Empty,
}

#[derive(Debug, Clone)]
pub struct NamedRuleBody {
    pub name: N<Ident>,
    pub body: N<RuleBody>,
}

#[derive(Debug, Clone)]
pub struct RuleBody {
    pub body: Vec<N<RuleElement>>,
}

#[derive(Debug, Clone)]
pub struct RuleElement {
    pub name: Option<N<Ident>>,
    pub nt: N<Ident>,
    pub quantifier: Option<N<Quantifier>>,
}

#[derive(Debug, Clone)]
pub enum Quantifier {
    /// ?
    Maybe,
    /// *
    Multi,
    /// +
    AtLeastOne,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
}
