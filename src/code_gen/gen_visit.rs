use indexmap::set::IndexSet;
use iterable::Iterable;
use itertools::Itertools;

use crate::ast;
use crate::ast::{Ident, N};
use crate::code_gen::CodeGen;
use crate::util::{indent, trim};

impl<'ast> CodeGen<'ast> {
    pub fn gen_visit(&self, is_mut: bool) -> String {
        let body = self
            .mir
            .rules
            .lazy_map(|r| self.gen_visit_method(r, is_mut))
            .join("\n\n");
        let body_leaf = self.gen_visit_method_empty_multi(&self.mir.leaf_nodes, is_mut);
        let body_reserved = self.gen_visit_method_empty_multi(&self.mir.reserved_nodes, is_mut);
        let body_std_primary =
            self.gen_visit_method_empty_multi(&self.mir.std_primary_nodes, is_mut);
        let visitor = format!(
            r#"
use crate::ast::*;

pub trait {}: Sized {{
{}

{}

{}

{}
}}
        "#,
            visitor_name(is_mut),
            indent(&body),
            indent(&body_leaf),
            indent(&body_reserved),
            indent(&body_std_primary)
        );

        let walk_methods = self
            .mir
            .rules
            .lazy_map(|r| self.gen_walk_method(r, is_mut))
            .join("\n\n");

        format!("{}\n\n{}\n", trim(&visitor), trim(&walk_methods))
    }

    fn gen_visit_method(&self, rule: &ast::Rule, is_mut: bool) -> String {
        let visit_name = visit_name(&rule.name);
        let walk_name = walk_name(&rule.name);

        let ty = self.node_type_name(&rule.name);
        let ret = format!(
            r#"
fn {}(&mut self, n: {}) {{
    {}(self, n);
}}
        "#,
            visit_name,
            wrap_mut(is_mut, &ty),
            walk_name
        );
        trim(&ret)
    }

    fn gen_visit_method_empty_multi(&self, nodes: &IndexSet<&N<Ident>>, is_mut: bool) -> String {
        nodes
            .iter()
            .map(|n| self.gen_visit_method_empty(n, is_mut))
            .join("\n\n")
    }

    fn gen_visit_method_empty(&self, id: &Ident, is_mut: bool) -> String {
        let visit_name = visit_name(id);
        let ty = self.node_type_name(id);
        format!(
            "fn {}(&mut self, _n: {}) {{}}",
            visit_name,
            wrap_mut(is_mut, &ty)
        )
    }

    fn gen_walk_method(&self, rule: &ast::Rule, is_mut: bool) -> String {
        use ast::RuleKind::*;

        let walk_name = walk_name(&rule.name);
        let ty = self.node_type_name(&rule.name);
        let body = match &rule.kind {
            Enum(s) => {
                let ty_name = self.type_name(&rule.name);
                let matches = s.lazy_map(|d| self.gen_visit_enum(d)).join("\n");
                let match_target = deref_mut(self.mir.is_boxed(&rule.name), is_mut, "n");
                format!(
                    "use {}::*;\nmatch {} {{\n{}\n}}",
                    ty_name,
                    match_target,
                    indent(&matches)
                )
            }
            Normal(s) => {
                let prefix = if is_mut { "&mut n." } else { "&n." };
                self.gen_visit_struct(s, prefix)
            }
        };
        let ret = format!(
            r#"
#[allow(unused)]
pub fn {}<'ast, V: {}>(v: &mut V, n: {}) {{
{}
}}
        "#,
            walk_name,
            visitor_name(is_mut),
            wrap_mut(is_mut, &ty),
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

fn visitor_name(is_mut: bool) -> &'static str {
    if is_mut {
        "VisitorMut"
    } else {
        "Visitor<'ast>"
    }
}

fn wrap_mut(is_mut: bool, ty: &str) -> String {
    if is_mut {
        format!("&mut {}", ty)
    } else {
        format!("&'ast {}", ty)
    }
}

fn deref_mut(is_boxed: bool, is_mut: bool, variable: &str) -> String {
    let x = match (is_boxed, is_mut) {
        (false, false) => "&**",
        (false, true) => "&mut **",
        (true, false) => "&***",
        (true, true) => "&mut***",
    };
    format!("{}{}", x, variable)
}
