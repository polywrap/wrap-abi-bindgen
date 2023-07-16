use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::is_keyword::_is_keyword;

handlebars_helper!(serde_keyword: |val: Value| {
    let type_val = val.as_str().unwrap();
    _serde_keyword(type_val)
});

pub fn _serde_keyword(type_val: &str) -> String {
    if _is_keyword(type_val) {
        //  return `#[serde(rename = "${type}")]\n    `;
        return format!("#[serde(rename = \"{}\")]\n    ", type_val);
    }
    "".to_string()
}
