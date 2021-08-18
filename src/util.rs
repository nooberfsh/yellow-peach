pub fn is_std_primary(input: &str) -> bool {
    match input {
        "bool" | "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128"
        | "char" | "str" | "!" => true,
        _ => false,
    }
}
