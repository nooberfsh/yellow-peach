use iterable::Iterable;

use crate::ast::Ident;
use crate::util::is_std_primary;

pub mod gen_ast;
pub mod gen_meta;

fn type_name(input: &Ident) -> String {
    let s = input.to_str();
    if is_std_primary(s) {
        s.to_string()
    } else {
        camel_case(s)
    }
}

fn node_type_name(input: &Ident) -> String {
    format!("N<{}>", type_name(input))
}

fn trim(s: &str) -> String {
    s.trim().to_string()
}

fn indent(s: &str) -> String {
    let lines: Vec<_> = s.lines().collect();
    lines.map(|s| format!("    {}", s)).join("\n")
}

fn camel_case(s: &str) -> String {
    let chars: Vec<_> = s.chars().collect();
    let mut buf = Vec::with_capacity(chars.len());
    let mut is_begin = true;
    for c in chars {
        if is_begin {
            buf.push(c.to_ascii_uppercase());
            is_begin = false;
        } else if c == '_' {
            is_begin = true
        } else {
            buf.push(c.to_ascii_lowercase())
        }
    }
    buf.into_iter().collect()
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
