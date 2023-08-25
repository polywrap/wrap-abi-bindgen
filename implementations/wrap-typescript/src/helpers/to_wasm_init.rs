use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::util::map_types;

use super::to_wasm::to_wasm_fn;
use super::detect_keyword::detect_keyword_fn;

pub fn to_wasm_init_fn(value: &str) -> String {
    let mut type_str = String::from(value);

    if type_str.chars().last() == Some('!') {
        type_str = (type_str[0..type_str.len() - 1]).to_string();
    } else {
        let null_type = to_wasm_fn(&type_str.clone());
        let null_optional = "| null";

        if null_type.ends_with(null_optional) {
            return "null".to_string();
        }
    }

    if type_str.starts_with('[') {
        return "[]".to_string();
    }

    if type_str.starts_with("Map<") {
        let (key_type, val_type) = map_types(&type_str).unwrap();
        let wasm_key_type = to_wasm_fn(&key_type);
        let wasm_val_type = to_wasm_fn(&val_type);
        return format!("new Map<{}, {}>()", wasm_key_type, wasm_val_type);
    }

    match type_str.as_str() {
        "Int" | "Int8" | "Int16" | "Int32" | "UInt" | "UInt8" | "UInt16" | "UInt32" => {
            "0".to_string()
        }
        "String" => "\"\"".to_string(),
        "Boolean" => "false".to_string(),
        "Bytes" => "new ArrayBuffer(0)".to_string(),
        "BigInt" => "BigInt.fromUInt16(0)".to_string(),
        "BigNumber" => "new BigNumber(BigInt.fromUInt16(0), 0, 0)".to_string(),
        "JSON" => "\"\"".to_string(),
        _ => {
            if type_str.contains("Enum_") {
                "0".to_string()
            } else {
                type_str = detect_keyword_fn(&type_str);
                format!("new Types.{}()", type_str)
            }
        }
    }
}

handlebars_helper!(to_wasm_init: |value: Value| {
    let value_str = value.as_str().unwrap();
    to_wasm_init_fn(value_str)
});
