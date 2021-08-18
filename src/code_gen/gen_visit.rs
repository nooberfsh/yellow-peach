use iterable::Iterable;

use crate::ast;
use crate::ast::{Ident, N};
use crate::code_gen::CodeGen;
use crate::util::{indent, trim};
use std::collections::HashSet;

impl<'ast> CodeGen<'ast> {
    pub fn gen_visit(&self) -> String {
        let body = self
            .mir
            .rules
            .lazy_map(|r| self.gen_visit_method(r))
            .join("\n\n");
        let body_leaf = self.gen_visit_method_empty_multi(&self.mir.leaf_nodes);
        let body_reserved = self.gen_visit_method_empty_multi(&self.mir.reserved_nodes);
        let body_std_primary = self.gen_visit_method_empty_multi(&self.mir.std_primary_nodes);
        let visitor = format!(
            r#"
use crate::ast::*;

pub trait Visitor<'ast>: Sized {{
{}

{}

{}

{}
}}
        "#,
            indent(&body),
            indent(&body_leaf),
            indent(&body_reserved),
            indent(&body_std_primary)
        );

        let walk_methods = self
            .mir
            .rules
            .lazy_map(|r| self.gen_walk_method(r))
            .join("\n\n");

        format!("{}\n\n{}\n", trim(&visitor), trim(&walk_methods))
    }

    fn gen_visit_method(&self, rule: &ast::Rule) -> String {
        let visit_name = visit_name(&rule.name);
        let walk_name = walk_name(&rule.name);

        let ty = self.node_type_name(&rule.name);
        let ret = format!(
            r#"
fn {}(&mut self, n: &'ast {}) {{
    {}(self, n);
}}
        "#,
            visit_name, ty, walk_name
        );
        trim(&ret)
    }

    fn gen_visit_method_empty_multi(&self, nodes: &HashSet<&N<Ident>>) -> String {
        nodes
            .lazy_map(|n| self.gen_visit_method_empty(n))
            .join("\n\n")
    }

    fn gen_visit_method_empty(&self, id: &Ident) -> String {
        let visit_name = visit_name(id);
        let ty = self.node_type_name(id);
        format!("fn {}(&mut self, _n: &'ast {}) {{}}", visit_name, ty)
    }

    fn gen_walk_method(&self, rule: &ast::Rule) -> String {
        use ast::RuleKind::*;

        let walk_name = walk_name(&rule.name);
        let ty = self.node_type_name(&rule.name);
        let body = match &rule.kind {
            Enum(s) => {
                let ty_name = self.type_name(&rule.name);
                let matches = s.lazy_map(|d| self.gen_visit_enum(d)).join("\n");
                let match_target = if self.mir.is_boxed(&rule.name) {
                    "&***n"
                } else {
                    "&**n"
                };
                format!(
                    "use {}::*;\nmatch {} {{\n{}\n}}",
                    ty_name,
                    match_target,
                    indent(&matches)
                )
            }
            Normal(s) => self.gen_visit_struct(s, "&n."),
        };
        let ret = format!(
            r#"
#[allow(unused)]
pub fn {}<'a, V: Visitor<'a>>(v: &mut V, n: &'a {}) {{
{}
}}
        "#,
            walk_name,
            ty,
            indent(&body)
        );
        trim(&ret)
    }

    fn gen_visit_enum(&self, body: &ast::NamedRuleBody) -> String {
        let variant = self.variant_name(&body.name);
        let ret = if let Some(body) = &body.body {
            let mut variables = Vec::with_capacity(body.body.len());
            for i in 0..body.body.len() {
                variables.push(format!("a{}", i));
            }
            let body = self.gen_visit_variable(body, &variables);
            let match_variables = variables.join(", ");
            format!(
                r#"
{}({}) => {{
{}
}},
            "#,
                variant,
                match_variables,
                indent(&body)
            )
        } else {
            format!("{} => {{}},", variant)
        };
        trim(&ret)
    }

    fn gen_visit_variable(&self, body: &ast::RuleBody, variables: &[String]) -> String {
        assert_eq!(body.body.len(), variables.len());
        (&body.body)
            .lazy_zip(variables)
            .lazy_map(|(e, v)| self.gen_visit_element(e, v))
            .join("\n")
    }

    fn gen_visit_struct(&self, body: &ast::RuleBody, prefix: &str) -> String {
        (&body.body)
            .lazy_map(|e| self.gen_visit_field(e, prefix))
            .join("\n")
    }

    fn gen_visit_field(&self, ele: &ast::RuleElement, prefix: &str) -> String {
        let field_name = self.field_name(ele);
        let field_name = format!("{}{}", prefix, field_name);
        self.gen_visit_element(ele, &field_name)
    }

    fn gen_visit_element(&self, ele: &ast::RuleElement, variable: &str) -> String {
        use ast::Quantifier::*;
        let visit_name = visit_name(&ele.nt);
        let ret = if let Some(d) = &ele.quantifier {
            match &d.t {
                Maybe => {
                    format!(
                        r#"
if let Some(d) = {} {{
    v.{}(d);
}}
                "#,
                        variable, visit_name
                    )
                }
                Multi | AtLeastOne => {
                    format!(
                        r#"
for d in {} {{
    v.{}(d);
}}
                "#,
                        variable, visit_name
                    )
                }
            }
        } else {
            format!("v.{}({});", visit_name, variable)
        };
        trim(&ret)
    }
}

fn walk_name(id: &Ident) -> String {
    format!("walk_{}", id.to_str())
}

fn visit_name(id: &Ident) -> String {
    format!("visit_{}", id.to_str())
}
