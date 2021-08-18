use iterable::Iterable;

use crate::ast;
use crate::util::{indent, is_keyword};

use super::*;

impl<'ast> CodeGen<'ast> {
    pub fn gen_ast(&self) -> String {
        (&self.mir.rules).lazy_map(|r| self.gen_rule(r)).join("\n\n")
    }

    fn gen_rule(&self, rule: &ast::Rule) -> String {
        use ast::RuleKind::*;
        let ty_name = type_name(&rule.name);
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
        let variant = type_name(&body.name);
        if let Some(body) = &body.body {
            let body = (&body.body).lazy_map(|r| self.quantifier_type(r)).join(", ");
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
            format!("N<Box<{}>>", type_name(input))
        } else {
            format!("N<{}>", type_name(input))
        }
    }

}
