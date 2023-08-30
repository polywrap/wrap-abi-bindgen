use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(array_length: |arr: Value| {
    arr.as_array().unwrap().len()
});
