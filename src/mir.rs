use std::collections::{HashMap, HashSet};

use iterable::Iterable;

use crate::ast;
use crate::ast::{Grammar, Ident, N};
use crate::util::is_std_primary;
use crate::visit::{walk_rule, Visitor};

#[derive(Debug, Clone)]
pub struct Mir<'ast> {
    pub boxed_rules: Vec<&'ast N<Ident>>,
    pub rule_map: HashMap<String, &'ast N<ast::Rule>>,
    pub rules: &'ast Vec<N<ast::Rule>>,
    pub leaf_nodes: HashSet<&'ast N<Ident>>,
    pub reserved_nodes: HashSet<&'ast N<ast::Ident>>,
    pub std_primary_nodes: HashSet<&'ast N<ast::Ident>>,
}

#[derive(Debug, Clone)]
pub enum Error<'ast> {
    BasicCheckError(Vec<&'ast N<Ident>>, Vec<&'ast N<ast::Attr>>),
}

impl<'ast> Mir<'ast> {
    pub fn is_boxed(&self, id: &Ident) -> bool {
        (&self.boxed_rules)
            .find(|r| r.to_str() == id.to_str())
            .is_some()
    }
}

pub fn lower(grammar: &N<ast::Grammar>) -> Result<Mir<'_>, Error<'_>> {
    // basic check
    let mut bc = BasicCheck::new();
    bc.visit_grammar(grammar);
    bc.into_error()?;

    let mut builder = MirBuilder::new(grammar);
    builder.visit_grammar(grammar);

    let ret = builder.build();
    Ok(ret)
}

// basic check
#[derive(Debug, Clone)]
struct BasicCheck<'ast> {
    invalid_attrs: Vec<&'ast N<ast::Attr>>,
    invalid_ids: Vec<&'ast N<Ident>>,
}

static ATTR_BOX: &str = "box";
static ALLOWED_ATTRS: &[&str] = &[ATTR_BOX];

static RESERVED: &[&str] = &["string"];

impl<'ast> BasicCheck<'ast> {
    fn new() -> Self {
        BasicCheck {
            invalid_attrs: vec![],
            invalid_ids: vec![],
        }
    }

    fn into_error(self) -> Result<(), Error<'ast>> {
        if self.invalid_ids.is_empty() && self.invalid_attrs.is_empty() {
            Ok(())
        } else {
            Err(Error::BasicCheckError(self.invalid_ids, self.invalid_attrs))
        }
    }
}

impl<'ast> Visitor<'ast> for BasicCheck<'ast> {
    fn visit_ident(&mut self, n: &'ast N<Ident>) {
        let s = n.to_str().chars().all(|c| c == '_' || c.is_lowercase() || c.is_numeric());
        if !s {
            self.invalid_ids.push(n)
        }
    }

    fn visit_attr(&mut self, n: &'ast N<ast::Attr>) {
        let name = n.to_str();
        if !ALLOWED_ATTRS.contains(&name) {
            self.invalid_attrs.push(n)
        }
    }
}

// basic check
#[derive(Debug, Clone)]
struct MirBuilder<'ast> {
    boxed_rules: Vec<&'ast N<Ident>>,
    rule_map: HashMap<String, &'ast N<ast::Rule>>,
    rules: &'ast Vec<N<ast::Rule>>,
    leaf_nodes: HashSet<&'ast N<ast::Ident>>,
    reserved_nodes: HashSet<&'ast N<ast::Ident>>,
    std_primary_nodes: HashSet<&'ast N<ast::Ident>>,
}

impl<'ast> MirBuilder<'ast> {
    fn new(grammar: &'ast N<Grammar>) -> Self {
        let mut rule_map = HashMap::new();
        for r in &grammar.rules {
            let name = r.name.to_str().to_string();
            rule_map.insert(name, r);
        }
        MirBuilder {
            rule_map,
            rules: &grammar.rules,
            leaf_nodes: HashSet::new(),
            reserved_nodes: HashSet::new(),
            std_primary_nodes: HashSet::new(),
            boxed_rules: vec![],
        }
    }

    fn build(self) -> Mir<'ast> {
        Mir {
            boxed_rules: self.boxed_rules,
            rule_map: self.rule_map,
            rules: self.rules,
            leaf_nodes: self.leaf_nodes,
            reserved_nodes: self.reserved_nodes,
            std_primary_nodes: self.std_primary_nodes,
        }
    }
}

impl<'ast> Visitor<'ast> for MirBuilder<'ast> {
    fn visit_rule(&mut self, n: &'ast N<ast::Rule>) {
        for attr in &n.attrs {
            if attr.to_str() == ATTR_BOX {
                self.boxed_rules.push(&n.name);
                break;
            }
        }
        walk_rule(self, n)
    }

    // ignore enum name
    fn visit_named_rule_body(&mut self, n: &'ast N<ast::NamedRuleBody>) {
        if let Some(d) = &n.body {
            self.visit_rule_body(d);
        }
    }

    fn visit_ident(&mut self, n: &'ast N<Ident>) {
        let name = n.to_str();
        if is_std_primary(name) {
            self.std_primary_nodes.insert(n);
        } else if RESERVED.contains(&name) {
            self.reserved_nodes.insert(n);
        } else if !self.rule_map.contains_key(name) {
            self.leaf_nodes.insert(n);
        }
    }
}
