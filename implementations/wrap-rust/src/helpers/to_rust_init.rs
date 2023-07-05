use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::to_rust::_to_rust;
use crate::helpers::util::map_types;

handlebars_helper!(to_rust_init: |value: Value| {
    let value_str = value.as_str().unwrap();
    _to_rust_init(value_str)
});

pub fn _to_rust_init(value: &str) -> String {
    let mut type_str = String::from(value);
    let mut optional = false;

    if type_str.ends_with("!") {
        type_str.pop();
    } else {
        optional = true;
    }

    if type_str.starts_with("[") {
        return optional_modifier("vec![]", optional);
    }

    if type_str.starts_with("Map<") {
        let (key_type, val_type) = map_types(value).unwrap();
        let rs_key_type = _to_rust(&key_type);
        let rs_val_type = _to_rust(&val_type);
        return optional_modifier(&format!("Map::<{}, {}>::new()", rs_key_type, rs_val_type), optional);
    }

    match type_str.as_str() {
        "Int" | "Int8" | "Int16" | "Int32" | "Int64" |
        "UInt" | "UInt8" | "UInt16" | "UInt32" | "UInt64" => optional_modifier("0", optional),
        "String" => optional_modifier("String::new()", optional),
        "Boolean" => optional_modifier("false", optional),
        "Bytes" => optional_modifier("vec![]", optional),
        "BigInt" => optional_modifier("BigInt::default()", optional),
        "BigNumber" => optional_modifier("BigNumber::default()", optional),
        "JSON" => optional_modifier("JSON::Value::Null", optional),
        _ => {
            let rs_type = _to_rust(&type_str);
            if type_str.starts_with("Enum_") {
                optional_modifier(&format!("{}::_MAX_", rs_type), optional)
            } else {
                optional_modifier(&format!("{}::new()", rs_type), optional)
            }
        }
    }
}

fn optional_modifier(str: &str, optional: bool) -> String {
    if optional {
        "None".to_string()
    } else {
        str.to_string()
    }
}