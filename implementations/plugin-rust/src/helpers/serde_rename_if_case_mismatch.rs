use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::is_keyword::_is_keyword;

handlebars_helper!(serde_rename_if_case_mismatch: |val: Value| {
    let name = val.as_str().unwrap();
    _serde_rename_if_case_mismatch(name)
});

pub fn _serde_rename_if_case_mismatch(val: &str) -> String {
    if has_uppercase(val) || _is_keyword(val) {
        return format!("#[serde(rename = \"{}\")]\n    ", val);
    }
    "".to_string()
}

fn has_uppercase(value: &str) -> bool {
    value != value.to_lowercase()
}