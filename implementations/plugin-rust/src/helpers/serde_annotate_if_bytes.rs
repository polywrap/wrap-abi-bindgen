use handlebars::handlebars_helper;
use serde_json::Value;

handlebars_helper!(serde_annotate_if_bytes: |val: Value| {
    let name = val.as_str().unwrap();
    _serde_annotate_if_wrapper(name)
});

pub fn _serde_annotate_if_wrapper(val: &str) -> String {
    match val {
        "Bytes" =>  "#[serde(with = \"bytes\")]\n    ".to_string(),
        "JSON" =>  "#[serde(with = \"json\")]\n    ".to_string(),
        "BigInt" =>  "#[serde(with = \"bigint\")]\n    ".to_string(),
        _ => "".to_string(),
    }
}
