use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(pretty: |value: Value| {
    let mut json_str = serde_json::to_string(&value).unwrap();
    json_str = json_str.replace("\n", "\\n"); // escape backslashes
    let value = serde_json::from_str::<Value>(&json_str).unwrap();
    serde_json::to_string_pretty(&value).unwrap()
});
