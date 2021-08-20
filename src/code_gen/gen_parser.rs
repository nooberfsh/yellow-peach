use std::collections::HashSet;

use iterable::Iterable;

use crate::util::{trim, indent};
use crate::code_gen::CodeGen;
use crate::ast::{Ident, N};

impl<'ast> CodeGen<'ast> {
    pub fn gen_parse(&self) -> String {
        let body = self
            .mir
            .rules
            .lazy_map(|r| self.gen_parse_method(&r.name))
            .join("\n\n");
        let body_leaf = self.gen_parse_method_multi(&self.mir.leaf_nodes);
        let body_reserved = self.gen_parse_method_multi(&self.mir.reserved_nodes);
        let body_std_primary =
            self.gen_parse_method_multi(&self.mir.std_primary_nodes);
        let ret = format!(
            r#"
use crate::parser::{{Parser, Result}};
use crate::ast::*;
use crate::token::TokenKind;

impl Parser {{
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
        format!("{}\n", ret)
    }

    fn gen_parse_method_multi(&self, nodes: &HashSet<&N<Ident>>) -> String {
        nodes
            .lazy_map(|n| self.gen_parse_method(n))
            .join("\n\n")
    }

    fn gen_parse_method(&self, id: &Ident) -> String {
        let parse_name = parse_name(id);
        let ty = self.node_type_name(id);
        let ret = format!(
            r#"
pub fn {}(&mut self) -> Result<{}> {{
    self.parse(|parser| {{
        todo!()
    }})
}}
        "#,
            parse_name,
            ty
        );
        trim(&ret)
    }
}

fn parse_name(id: &Ident) -> String {
    format!("parse_{}", id.to_str())
}
