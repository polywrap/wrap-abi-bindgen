use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(nullable_default: |value: Value| {
    let type_val = value.as_str().unwrap();
    _nullable_default(type_val)
});

pub fn _nullable_default(value: &str) -> String {
    if value.ends_with("?") {
        format!("{} = null", value)
    } else {
        value.to_string()
    }
}
