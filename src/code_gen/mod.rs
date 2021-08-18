use crate::ast::Ident;
use crate::util::{is_std_primary, camel_case};
use crate::mir::Mir;

pub mod gen_ast;
pub mod gen_meta;

pub struct CodeGen<'ast> {
    mir: Mir<'ast>,
}

impl<'ast> CodeGen<'ast> {
    pub fn new(mir: Mir<'ast>) -> Self {
        CodeGen{mir}
    }
}
