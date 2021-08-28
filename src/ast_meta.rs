use std::cmp::Eq;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ops::DerefMut;

use reacto::span::Span;

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

impl<T> Deref for N<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for N<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<T: PartialEq> PartialEq for N<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<T: Eq> Eq for N<T> {}

impl<T: Hash> Hash for N<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state)
    }
}
