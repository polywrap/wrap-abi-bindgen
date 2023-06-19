use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(is_keyword: |val: Value| {
    let s = val.as_str().unwrap();
    _is_keyword(s)
});

pub fn _is_keyword(s: &str) -> bool {
    match s {
        "break" | "case" | "catch" | "class" | "const" | "continue" | "debugger"
        | "default" | "delete" | "do" | "else" | "export" | "extends" | "false"
        | "finally" | "for" | "function" | "if" | "import" | "in" | "instanceof"
        | "new" | "null" | "return" | "super" | "switch" | "this" | "throw"
        | "true" | "try" | "typeof" | "var" | "void" | "while" | "with" | "yield"
        | "let" | "await" | "enum" | "implements" | "interface" | "package"
        | "private" | "protected" | "public" | "static" | "arguments" | "eval" => true,
        _ => false,
    }
}

