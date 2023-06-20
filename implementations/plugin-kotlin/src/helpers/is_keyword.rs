use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(is_keyword: |val: Value| {
    let s = val.as_str().unwrap();
    _is_keyword(s)
});

pub fn _is_keyword(s: &str) -> bool {
    match s {
        "as" | "as?" | "break" | "class" | "continue" | "do" | "else" | "false"
        | "for" | "fun" | "if" | "in" | "!in" | "interface" | "is" | "!is"
        | "null" | "object" | "package" | "return" | "super" | "this" | "throw"
        | "true" | "try" | "typealias" | "typeof" | "val" | "var" | "when"
        | "while" => true,
        _ => false,
    }
}


