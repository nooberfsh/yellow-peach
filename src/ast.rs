use crate::span::Span;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct NodeId(usize);

#[derive(Clone, Debug)]
pub struct IdGen {
    id: Arc<AtomicUsize>,
}

impl IdGen {
    pub fn new() -> Self {
        IdGen {
            id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn next(&self) -> NodeId {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        NodeId(id)
    }
}

#[derive(Debug, Clone)]
pub struct N<T> {
    pub span: Span,
    pub id: NodeId,
    pub t: T,
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
    Enum(Vec<(N<RuleBody>, N<Ident>)>),
    Normal(N<RuleBody>),
    Empty,
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
pub enum Quantifier{
    /// ?
    Maybe,
    /// *
    Multi,
    /// +
    AtLeastOne,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String
}
