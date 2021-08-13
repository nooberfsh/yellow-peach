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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let (start, end) = (1,3);
        let a = Span {start,end };
        let res = a.merge(a);
        assert_eq!(res, a);
    }
}
