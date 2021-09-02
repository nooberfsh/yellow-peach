use iterable::Iterable;

pub fn is_std_primary(input: &str) -> bool {
    match input {
        "bool" | "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128"
        | "f32" | "f64" | "usize" | "isize" | "char" | "str" | "!" => true,
        _ => false,
    }
}

// TODO: add all keywords
pub fn is_keyword(input: &str) -> bool {
    match input {
        "as" | "break" | "where" => true,
        _ => false,
    }
}

pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

pub fn indent(s: &str) -> String {
    let mut ret = vec![];
    for l in s.lines() {
        let d = if l.trim().is_empty() {
            String::new()
        } else {
            format!("    {}", l)
        };
        ret.push(d)
    }
    ret.join("\n")
}

pub fn camel_case(s: &str) -> String {
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
