use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(is_not_first: |index: Value| {
    let index_u64 = index.as_u64().unwrap();
    index_u64 != 0
});
