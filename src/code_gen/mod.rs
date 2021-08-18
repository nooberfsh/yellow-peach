use crate::ast::Ident;
use crate::mir::Mir;
use crate::util::{camel_case, is_std_primary};

pub mod gen_ast;
pub mod gen_meta;

pub struct CodeGen<'ast> {
    mir: Mir<'ast>,
}

impl<'ast> CodeGen<'ast> {
    pub fn new(mir: Mir<'ast>) -> Self {
        CodeGen { mir }
    }
}
