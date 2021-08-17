use iterable::Iterable;

use crate::ast;

use super::*;

pub fn gen_grammar(grammar: &ast::Grammar) -> String {
    (&grammar.rules).lazy_map(|r| gen_rule(r)).join("\n\n")
}

pub fn gen_rule(rule: &ast::Rule) -> String {
    use ast::RuleKind::*;
    let ty_name = type_name(&rule.name);
    let ret = match &rule.kind {
        Enum(s) => {
            let body = s.map(|b| gen_enum(b)).join(",\n");
            format!("pub enum {} {{\n{}\n}}", ty_name, indent(&body))
        }
        Normal(s) => {
            let body = gen_struct(s);
            format!("pub struct {} {{\n{}\n}}", ty_name, indent(&body))
        }
    };
    format!("#[derive(Clone, Debug)]\n{}", ret)
}

fn gen_struct(body: &ast::RuleBody) -> String {
    fn gen_field(ele: &ast::RuleElement) -> String {
        let name = match &ele.name {
            Some(d) => d.to_str(),
            None => ele.nt.to_str(),
        };
        let ty = quantifier_type(ele);
        format!("pub {}: {}", name, ty)
    }
    (&body.body).lazy_map(|e| gen_field(e)).join(",\n")
}

fn gen_enum(body: &ast::NamedRuleBody) -> String {
    let variant = type_name(&body.name);
    if let Some(body) = &body.body {
        let body = (&body.body).lazy_map(|r| quantifier_type(r)).join(", ");
        format!("{}({})", variant, body)
    } else {
        variant
    }
}

fn quantifier_type(ele: &ast::RuleElement) -> String {
    use ast::Quantifier::*;
    let ty = node_type_name(&ele.nt);
    if let Some(d) = &ele.quantifier {
        match &d.t {
            Multi | AtLeastOne => format!("Vec<{}>", ty),
            Maybe => format!("Option<{}>", ty),
        }
    } else {
        ty
    }
}
