use iterable::Iterable;

use crate::ast;
use crate::code_gen::gen_meta::gen_meta;
use crate::util::{indent, is_keyword};

use super::*;

impl<'ast> CodeGen<'ast> {
    pub fn gen_ast(&self) -> String {
        let meta = gen_meta();
        let body = (&self.mir.rules)
            .lazy_map(|r| self.gen_rule(r))
            .join("\n\n");
        let leaf_nodes = (&self.mir.leaf_nodes)
            .lazy_map(|r| self.gen_leaf_node(r))
            .join("\n\n");
        format!("{}\n{}\n\n{}", meta, body, leaf_nodes)
    }

    fn gen_leaf_node(&self, node: &Ident) -> String {
        let ty_name = self.type_name(node);
        let ret = format!("pub struct {};", ty_name);
        format!("#[derive(Clone, Debug, Copy)]\n{}", ret)
    }

    fn gen_rule(&self, rule: &ast::Rule) -> String {
        use ast::RuleKind::*;
        let ty_name = self.type_name(&rule.name);
        let ret = match &rule.kind {
            Enum(s) => {
                let body = s.map(|b| self.gen_enum(b)).join(",\n");
                format!("pub enum {} {{\n{}\n}}", ty_name, indent(&body))
            }
            Normal(s) => {
                let body = self.gen_struct(s);
                format!("pub struct {} {{\n{}\n}}", ty_name, indent(&body))
            }
        };
        format!("#[derive(Clone, Debug)]\n{}", ret)
    }

    fn gen_struct(&self, body: &ast::RuleBody) -> String {
        (&body.body).lazy_map(|e| self.gen_field(e)).join(",\n")
    }

    fn gen_field(&self, ele: &ast::RuleElement) -> String {
        let ty = self.quantifier_type(ele);
        let name = match &ele.name {
            Some(d) => d.to_str(),
            None => ele.nt.to_str(),
        };
        let name = if is_keyword(name) {
            format!("r#{}", name)
        } else {
            name.to_string()
        };
        format!("pub {}: {}", name, ty)
    }

    fn gen_enum(&self, body: &ast::NamedRuleBody) -> String {
        let variant = self.variant_name(&body.name);
        if let Some(body) = &body.body {
            let body = (&body.body)
                .lazy_map(|r| self.quantifier_type(r))
                .join(", ");
            format!("{}({})", variant, body)
        } else {
            variant
        }
    }

    fn quantifier_type(&self, ele: &ast::RuleElement) -> String {
        use ast::Quantifier::*;
        let ty = self.node_type_name(&ele.nt);
        if let Some(d) = &ele.quantifier {
            match &d.t {
                Multi | AtLeastOne => format!("Vec<{}>", ty),
                Maybe => format!("Option<{}>", ty),
            }
        } else {
            ty
        }
    }

    fn node_type_name(&self, input: &Ident) -> String {
        if self.mir.is_boxed(input) {
            format!("N<Box<{}>>", self.type_name(input))
        } else {
            format!("N<{}>", self.type_name(input))
        }
    }

    fn type_name(&self, input: &Ident) -> String {
        let s = input.to_str();
        if is_std_primary(s) {
            s.to_string()
        } else {
            camel_case(s)
        }
    }

    fn variant_name(&self, input: &Ident) -> String {
        let s = input.to_str();
        camel_case(s)
    }
}
