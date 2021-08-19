use crate::ast;
use crate::ast::Ident;
use crate::mir::Mir;
use crate::util::{camel_case, is_keyword, is_std_primary};

pub mod gen_ast;
pub mod gen_span;
pub mod gen_visit;

pub struct CodeGen<'ast> {
    mir: Mir<'ast>,
}

impl<'ast> CodeGen<'ast> {
    pub fn new(mir: Mir<'ast>) -> Self {
        CodeGen { mir }
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

    fn field_name(&self, ele: &ast::RuleElement) -> String {
        let name = match &ele.name {
            Some(d) => d.to_str(),
            None => ele.nt.to_str(),
        };
        let name = if ele.has_many() {
            format!("{}s", name)
        } else {
            name.to_string()
        };
        if is_keyword(&name) {
            format!("r#{}", name)
        } else {
            name
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
}
