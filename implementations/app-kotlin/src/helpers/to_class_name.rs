use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(to_class_name: |val: Value| {
    let type_val = val.as_str().unwrap();
    _to_class_name(type_val)
});

pub fn _to_class_name(value: &str) -> String {
    value
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(first_upper)
        .collect()
}

fn first_upper(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}