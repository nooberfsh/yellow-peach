use std::collections::HashMap;

use crate::ast::{Ident, N};
use crate::ast;
use crate::visit::{Visitor, walk_rule};
use crate::util::is_std_primary;

#[derive(Debug, Clone)]
pub struct Mir<'ast> {
    pub boxed_rules: Vec<&'ast N<Ident>>,
    pub rules: HashMap<String, &'ast N<ast::Rule>>,
    pub leaf_nodes: Vec<&'ast N<Ident>>,
}

#[derive(Debug, Clone)]
pub enum Error<'ast> {
    BasicCheckError(Vec<&'ast N<Ident>>, Vec<&'ast N<ast::Attr>>),
}

pub fn lower(grammar: & N<ast::Grammar>) -> Result<Mir<'_>, Error<'_>> {
    // basic check
    let mut bc = BasicCheck::new();
    bc.visit_grammar(grammar);
    bc.into_error()?;

    let mut rules = HashMap::new();
    for r in &grammar.rules {
       let name = r.name.to_str().to_string()        ;
        rules.insert(name, r);
    }
    let mut builder = MirBuilder::new(rules);
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

static RESERVED: &[&str]=  &["string"];

impl<'ast> BasicCheck<'ast> {
    fn new() -> Self {
        BasicCheck {
            invalid_attrs: vec![],
            invalid_ids: vec![],
        }
    }

    fn into_error(self) -> Result<(), Error<'ast>> {
        Err(Error::BasicCheckError(self.invalid_ids, self.invalid_attrs))
    }
}

impl<'ast> Visitor<'ast> for BasicCheck<'ast> {
    fn visit_ident(&mut self, n: &'ast N<Ident>) {
        let s = n.to_str().chars().all(|c| c.is_lowercase());
        if !s {
            self.invalid_ids.push(n)
        }
    }

    fn visit_attr(&mut self, n: &'ast N<ast::Attr>) {
        let name = n.to_str();
        if ALLOWED_ATTRS.contains(&name) {
            self.invalid_attrs.push(n)
        }
    }
}

// basic check
#[derive(Debug, Clone)]
struct MirBuilder<'ast> {
    boxed_rules: Vec<&'ast N<Ident>>,
    rules: HashMap<String, &'ast N<ast::Rule>>,
    leaf_nodes: Vec<&'ast N<ast::Ident>>,
}

impl<'ast> MirBuilder<'ast> {
    fn new(rules: HashMap<String, &'ast N<ast::Rule>>) -> Self {
        MirBuilder {
            rules,
            leaf_nodes: vec![],
            boxed_rules: vec![],
        }
    }

    fn build(self) -> Mir<'ast> {
        Mir {
            boxed_rules: self.boxed_rules,
            rules: self.rules,
            leaf_nodes: self.leaf_nodes,
        }
    }
}

impl<'ast> Visitor<'ast> for MirBuilder<'ast> {
    fn visit_rule(&mut self, n: &'ast N<ast::Rule>) {
        for attr in &n.attrs {
            if attr.to_str() == ATTR_BOX {
                self.boxed_rules.push(&n.name);
                break
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
            return
        }
        if RESERVED.contains(&name) {
            return
        }
        if !self.rules.contains_key(name) {
            self.leaf_nodes.push(n)
        }
    }
}
