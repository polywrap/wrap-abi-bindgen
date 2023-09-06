use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(pretty: |value: Value| {
    let str = serde_json::to_string_pretty(&value).unwrap();
    str.replace("\\", "\\") // escape backslashes
});
