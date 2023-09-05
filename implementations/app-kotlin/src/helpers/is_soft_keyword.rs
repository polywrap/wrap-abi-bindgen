use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(is_soft_keyword: |val: Value| {
    let s = val.as_str().unwrap();
    _is_soft_keyword(s)
});

pub fn _is_soft_keyword(s: &str) -> bool {
    match s {
        "by" | "catch" | "constructor" | "delegate" | "dynamic" | "field" | "file"
        | "finally" | "get" | "import" | "init" | "param" | "property" | "receiver"
        | "set" | "setparam" | "value" | "where" => true,
        _ => false,
    }
}

