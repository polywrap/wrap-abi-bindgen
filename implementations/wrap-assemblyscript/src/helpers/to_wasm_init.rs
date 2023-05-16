use handlebars::handlebars_helper;
use serde_json::{Value};

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
        let first_open_bracket_idx = type_str.find('<').unwrap_or_else(|| {
            panic!("Invalid Map: {}", type_str);
        });
        let last_close_bracket_idx = type_str
            .rfind('>')
            .unwrap_or_else(|| panic!("Invalid Map: {}", type_str));

        let key_val_types = type_str
            .get(first_open_bracket_idx + 1..last_close_bracket_idx)
            .unwrap_or_else(|| panic!("Invalid Map: {}", type_str));

        let first_comma_idx = key_val_types.find(',').unwrap_or_else(|| {
            panic!("Invalid Map: {}", type_str);
        });
        let key_type = key_val_types
            .get(0..first_comma_idx)
            .unwrap_or_else(|| panic!("Invalid Map: {}", type_str))
            .trim()
            .to_string();
        let val_type = key_val_types
            .get(first_comma_idx + 1..)
            .unwrap_or_else(|| panic!("Invalid Map: {}", type_str))
            .trim()
            .to_string();

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
        "JSON" => "JSON.Value.Null()".to_string(),
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
