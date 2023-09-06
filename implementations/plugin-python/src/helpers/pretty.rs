use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(pretty: |value: Value| {
    let json_str = serde_json::to_string(&value).unwrap();
    let escaped_json_str = json_str.replace("\\n", "\\\\n"); // escape backslashes
    let escaped_val = serde_json::from_str::<Value>(&escaped_json_str).unwrap();
    serde_json::to_string_pretty(&escaped_val).unwrap()
});
