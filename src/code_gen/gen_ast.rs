use iterable::Iterable;
use itertools::Itertools;

use crate::ast;
use crate::util::indent;

use super::*;

impl<'ast> CodeGen<'ast> {
    pub fn gen_ast(&self) -> String {
        let body = self.mir.rules.iter().map(|r| self.gen_rule(r)).join("\n\n");
        let leaf_nodes = self
            .mir
            .leaf_nodes
            .iter()
            .map(|r| self.gen_leaf_node(r))
            .join("\n\n");
        let head = "use reacto::ast::N;";
        format!("{}\n\n{}\n\n{}\n", head, body, leaf_nodes)
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
                let body = s.map(|b| self.gen_ast_enum(b)).join(",\n");
                format!("pub enum {} {{\n{}\n}}", ty_name, indent(&body))
            }
            Normal(s) => {
                let body = self.gen_ast_struct(s);
                format!("pub struct {} {{\n{}\n}}", ty_name, indent(&body))
            }
        };
        format!("#[derive(Clone, Debug)]\n{}", ret)
    }

    fn gen_ast_struct(&self, body: &ast::RuleBody) -> String {
        (&body.body).lazy_map(|e| self.gen_ast_field(e)).join(",\n")
    }

    fn gen_ast_field(&self, ele: &ast::RuleElement) -> String {
        let ty = self.quantifier_type(ele);
        let name = self.field_name(ele);
        format!("pub {}: {}", name, ty)
    }

    fn gen_ast_enum(&self, body: &ast::NamedRuleBody) -> String {
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
}
