use handlebars::handlebars_helper;
use serde_json::{Value};

use super::is_keyword::is_keyword_fn;

// check if any of the keywords match the property name;
// if there's a match, insert `_` at the beginning of the property name.
pub fn detect_keyword_fn(value: &str) -> String {
    if is_keyword_fn(value) {
        format!("_{}", value)
    } else {
        String::from(value)
    }
}

handlebars_helper!(detect_keyword: |value: Value| {
    let value_str = value.as_str().unwrap();
    detect_keyword_fn(value_str)
});
