use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::is_keyword::_is_keyword;

handlebars_helper!(detect_keyword: |val: Value| {
    let type_val = val.as_str().unwrap();
    _detect_keyword(type_val)
});

pub fn _detect_keyword(type_val: &str) -> String {
    if _is_keyword(type_val) {
        return format!("_{}", type_val);
    }
    type_val.to_string()
}