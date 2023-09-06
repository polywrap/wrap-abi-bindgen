use std::collections::HashSet;
use handlebars::handlebars_helper;
use serde_json::{Value};

lazy_static! {
    static ref BASE_TYPES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("u8");
        set.insert("u16");
        set.insert("u32");
        set.insert("i8");
        set.insert("i16");
        set.insert("i32");
        set.insert("string");
        set.insert("bool");
        set
    };
}

pub fn is_base_type_fn(value: &str) -> bool {
    BASE_TYPES.contains(value)
}

handlebars_helper!(is_base_type: |value: Value| {
    let value_str = value.as_str().unwrap();
    is_base_type_fn(value_str)
});
