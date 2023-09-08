use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(has_required: |array: Value| {
    let arr = array.as_array().unwrap();
    arr.iter().any(|item|
        item.as_object()
        .and_then(|obj| obj.get("required"))
        .and_then(|required| required.as_bool())
        .unwrap_or(false)
    )
});
