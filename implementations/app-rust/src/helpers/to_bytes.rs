use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(to_bytes: |arr: Value| {
    let bytes = arr
    .as_array()
    .expect("Expected byte array, but the serde_json::Value was not of type array")
    .iter()
    .map(|v| {
        v.as_u64().unwrap().to_string()
    })
    .collect::<Vec<String>>()
    .join(",");

    bytes
});
