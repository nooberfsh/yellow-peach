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

fn type_name(input: &Ident) -> String {
    let s = input.to_str();
    if is_std_primary(s) {
        s.to_string()
    } else {
        camel_case(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!("Ab", camel_case("ab"));
        assert_eq!("Ab", camel_case("aB"));
        assert_eq!("AbAb", camel_case("aB_ab"));
        assert_eq!("AbAb", camel_case("aB_aB"));
    }
}
