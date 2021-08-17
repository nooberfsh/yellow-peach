use crate::code_gen::trim;

const META: &str = r#"
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct NodeId(pub(crate) usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start < end, "span start must less then end");
        Span { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn merge(&self, other: Span) -> Span {
        let start = self.start;
        let end = other.end;
        Self::new(start, end)
    }
}

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

impl<T> Deref for N<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}
"#;

pub fn gen_meta() -> String {
    trim(META)
}
