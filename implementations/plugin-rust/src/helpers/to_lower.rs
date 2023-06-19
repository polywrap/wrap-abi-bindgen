use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::util::{insert_at, replace_at};

handlebars_helper!(to_lower: |val: Value| {
    let str = val.as_str().unwrap();
    _to_lower(str)
});

pub fn _to_lower(s: &str) -> String {
    let mut result = s.to_string();
    for (i, char) in result.clone().chars().enumerate() {
        if char.is_uppercase() {
            let replace_char = char.to_lowercase().collect::<String>();
            result = replace_at(&result, i, &replace_char);
            if i != 0 && s.chars().nth(i - 1).unwrap() != '_' {
                result = insert_at(&result, i, "_");
            }
        }
    }
    result
}
