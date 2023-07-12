use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(serde_annotate_if_bytes: |val: Value| {
    let name = val.as_str().unwrap();
    _serde_annotate_if_bytes(name)
});

pub fn _serde_annotate_if_bytes(val: &str) -> String {
    if val == "Bytes" {
        return "#[serde(with = \"bytes\")]\n    ".to_string();
    }
    "".to_string()
}
