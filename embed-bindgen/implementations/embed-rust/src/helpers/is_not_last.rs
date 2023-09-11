use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(is_not_last: |index: Value, array: Value| {
    let index_u64 = index.as_u64().unwrap();
    let array_len = array.as_array().unwrap().len() as u64;
    array_len != (index_u64 + 1)
});
