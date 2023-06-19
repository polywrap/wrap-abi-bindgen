use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::util::{remove_at, replace_at};

handlebars_helper!(to_upper: |val: Value| {
    let str = val.as_str().unwrap();
    _to_upper(str)
});

pub fn _to_upper(s: &str) -> String {
    let mut result = s.to_string();
    let first_char = result.chars().nth(0).unwrap();
    let replace_char = first_char.to_uppercase().collect::<String>();
    result = replace_at(&result, 0, &replace_char);
    let mut i = 0;
    while i < result.len() {
        if let Some('_') = result.chars().nth(i) {
            if let Some(next_char) = result.chars().nth(i + 1) {
                let replace_char = next_char.to_uppercase().collect::<String>();
                result = replace_at(&result, i + 1, &replace_char);
                result = remove_at(&result, i);
            }
        }
        i += 1;
    }
    result
}
