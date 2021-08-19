use crate::code_gen::CodeGen;

static SPAN: &str = include_str!("../span.rs");

impl<'ast> CodeGen<'ast> {
    pub fn gen_span(&self) -> String {
        SPAN.to_string()
    }
}
