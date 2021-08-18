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
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);
        Self::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let a = Span::new(1, 3);
        let res = a.merge(a);
        assert_eq!(res, a);

        let b = Span::new(0, 2);
        let res = a.merge(b);
        assert_eq!(res, Span::new(0, 3));

        let b = Span::new(0, 1);
        let res = a.merge(b);
        assert_eq!(res, Span::new(0, 3));

        let b = Span::new(0, 4);
        let res = a.merge(b);
        assert_eq!(res, Span::new(0, 4));

        let b = Span::new(1, 4);
        let res = a.merge(b);
        assert_eq!(res, Span::new(1, 4));

        let b = Span::new(3, 4);
        let res = a.merge(b);
        assert_eq!(res, Span::new(1, 4));
    }
}
