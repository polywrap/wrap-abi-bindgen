use handlebars::handlebars_helper;
use serde_json::{Value};

use super::is_base_type::is_base_type_fn;

pub fn apply_optional_fn(value: &str, optional: bool, is_enum: bool) -> String {
    if optional {
        if
            value.starts_with("Array") ||
            value.starts_with("string") ||
            (!is_enum && !is_base_type_fn(value))
        {
            return format!("{} | null", value);
        } else {
            return format!("Box<{}> | null", value);
        }
    } else {
        String::from(value)
    }
}

handlebars_helper!(apply_optional: |value: Value, optional: bool, is_enum: bool| {
    let value_str = value.as_str().unwrap();
    apply_optional_fn(value_str, optional, is_enum)
});
