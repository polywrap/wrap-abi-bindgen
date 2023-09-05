use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(array_has_length: |arr: Value| {
    arr.is_array() && arr.as_array().unwrap().len() > 0
});
