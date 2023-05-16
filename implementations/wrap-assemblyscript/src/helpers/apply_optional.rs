use handlebars::handlebars_helper;

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

handlebars_helper!(apply_optional: |value: str, optional: bool, is_enum: bool| {
    apply_optional_fn(&value, optional, is_enum)
});
