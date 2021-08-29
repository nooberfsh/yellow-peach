use reacto::ast::N;

use crate::ast::*;

pub trait Visitor<'ast>: Sized {
    fn visit_grammar(&mut self, n: &'ast N<Grammar>) {
        walk_grammar(self, n)
    }

    fn visit_rule(&mut self, n: &'ast N<Rule>) {
        walk_rule(self, n)
    }

    fn visit_named_rule_body(&mut self, n: &'ast N<NamedRuleBody>) {
        walk_named_rule_body(self, n)
    }

    fn visit_rule_body(&mut self, n: &'ast N<RuleBody>) {
        walk_rule_body(self, n)
    }

    fn visit_rule_element(&mut self, n: &'ast N<RuleElement>) {
        walk_rule_element(self, n)
    }

    fn visit_quantifier(&mut self, n: &'ast N<Quantifier>) {
        walk_quantifier(self, n)
    }

    fn visit_ident(&mut self, n: &'ast N<Ident>) {
        walk_ident(self, n)
    }

    fn visit_attr(&mut self, n: &'ast N<Attr>) {
        walk_attr(self, n)
    }
}

macro_rules! walk_list {
    ($visitor: expr, $method: ident, $list: expr) => {
        for elem in $list {
            $visitor.$method(elem)
        }
    };
}

pub fn walk_grammar<'a, V: Visitor<'a>>(v: &mut V, n: &'a N<Grammar>) {
    walk_list!(v, visit_rule, &n.rules);
}

pub fn walk_rule<'a, V: Visitor<'a>>(v: &mut V, n: &'a N<Rule>) {
    walk_list!(v, visit_attr, &n.attrs);
    v.visit_ident(&n.name);
    match &n.kind {
        RuleKind::Enum(b) => walk_list!(v, visit_named_rule_body, b),
        RuleKind::Normal(b) => v.visit_rule_body(b),
    }
}

pub fn walk_named_rule_body<'a, V: Visitor<'a>>(v: &mut V, n: &'a N<NamedRuleBody>) {
    v.visit_ident(&n.name);
    if let Some(d) = &n.body {
        v.visit_rule_body(d);
    }
}

pub fn walk_rule_body<'a, V: Visitor<'a>>(v: &mut V, n: &'a N<RuleBody>) {
    walk_list!(v, visit_rule_element, &n.body)
}

pub fn walk_rule_element<'a, V: Visitor<'a>>(v: &mut V, n: &'a N<RuleElement>) {
    if let Some(d) = &n.name {
        v.visit_ident(d);
    }
    v.visit_ident(&n.nt);
    if let Some(q) = &n.quantifier {
        v.visit_quantifier(q);
    }
}

pub fn walk_quantifier<'a, V: Visitor<'a>>(_v: &mut V, _n: &'a N<Quantifier>) {}

pub fn walk_ident<'a, V: Visitor<'a>>(_v: &mut V, _n: &'a N<Ident>) {}

pub fn walk_attr<'a, V: Visitor<'a>>(_v: &mut V, _n: &'a N<Attr>) {}
