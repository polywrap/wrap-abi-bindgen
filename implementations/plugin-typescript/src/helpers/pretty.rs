use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(pretty: |value: Value| {
    let pretty_print = |v: Value| serde_json::to_string_pretty(&v).unwrap();
    value.as_str()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .map_or_else(|| pretty_print(value), pretty_print)
});
