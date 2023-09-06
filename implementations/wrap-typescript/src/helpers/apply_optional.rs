use handlebars::handlebars_helper;
use serde_json::{Value};

pub fn apply_optional_fn(value: &str, optional: bool, is_enum: bool) -> String {
    if optional {
        return format!("{} | null", value);
    } else {
        String::from(value)
    }
}

handlebars_helper!(apply_optional: |value: Value, optional: bool, is_enum: bool| {
    let value_str = value.as_str().unwrap();
    apply_optional_fn(value_str, optional, is_enum)
});
