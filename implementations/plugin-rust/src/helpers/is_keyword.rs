use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(is_keyword: |val: Value| {
    let s = val.as_str().unwrap();
    _is_keyword(s)
});

pub fn _is_keyword(s: &str) -> bool {
    match s {
        "as" | "break" | "const" | "continue" | "crate"
        | "else" | "enum" | "extern" | "false" | "fn"
        | "for" | "if" | "impl" | "in" | "let" | "loop"
        | "match" | "mod" | "move" | "mut" | "pub" | "ref"
        | "return" | "self" | "Self" | "static" | "struct" | "super"
        | "trait" | "true" | "type" | "unsafe" | "use" | "where"
        | "while" | "async" | "await" | "dyn" | "abstract"
        | "become" | "box" | "Box" | "do" | "final" | "macro"
        | "override" | "priv" | "typeof" | "unsized"
        | "virtual" | "yield" | "try" | "macro_rules"
        | "union" => true,
        _ => false,
    }
}
