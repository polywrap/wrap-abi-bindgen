use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::util::{insert_at, replace_at};

handlebars_helper!(to_lower: |val: Value| {
    let str = val.as_str().unwrap();
    _to_lower(str)
});

pub fn _to_lower(s: &str) -> String {
    let mut result = s.to_string();
    let mut i = 0;
    while i < result.len() {
        let char = result.chars().nth(i).unwrap();
        if char.is_uppercase() {
            let lower = char.to_lowercase().collect::<String>();
            result = replace_at(&result, i, &lower);
            if i != 0 && result.chars().nth(i - 1).unwrap() != '_' {
                result = insert_at(&result, i, "_");
            }
        }
        i += 1;
    }
    result
}
