use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::is_keyword::_is_keyword;
use crate::helpers::is_soft_keyword::_is_soft_keyword;

handlebars_helper!(detect_keyword_strict: |val: Value| {
    let type_val = val.as_str().unwrap();
    _detect_keyword_strict(type_val)
});

pub fn _detect_keyword_strict(type_val: &str) -> String {
    if _is_keyword(type_val) || _is_soft_keyword(type_val) {
        return format!("_{}", type_val);
    }
    type_val.to_string()
}