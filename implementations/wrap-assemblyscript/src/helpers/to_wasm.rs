use handlebars::handlebars_helper;
use serde_json::{Value};

use super::to_wasm_array::to_wasm_array_fn;
use super::to_wasm_map::to_wasm_map_fn;
use super::detect_keyword::detect_keyword_fn;
use super::apply_optional::apply_optional_fn;

pub fn to_wasm_fn(value: &str) -> String {
    let mut type_str = String::from(value);
    let mut is_enum = false;
    let mut optional = false;

    if type_str.ends_with('!') {
        type_str = type_str.trim_end_matches('!').to_string();
    } else {
        optional = true;
    }

    if type_str.starts_with('[') {
        return to_wasm_array_fn(&type_str, optional).unwrap();
    }

    if type_str.starts_with("Map<") {
        return to_wasm_map_fn(&type_str, optional).unwrap();
    }

    match type_str.as_str() {
        "Int" => type_str = "i32".to_owned(),
        "Int8" => type_str = "i8".to_owned(),
        "Int16" => type_str = "i16".to_owned(),
        "Int32" => type_str = "i32".to_owned(),
        "UInt" | "UInt32" => type_str = "u32".to_owned(),
        "UInt8" => type_str = "u8".to_owned(),
        "UInt16" => type_str = "u16".to_owned(),
        "String" => type_str = "string".to_owned(),
        "Boolean" => type_str = "bool".to_owned(),
        "Bytes" => type_str = "ArrayBuffer".to_owned(),
        "BigInt" => type_str = "BigInt".to_owned(),
        "BigNumber" => type_str = "BigNumber".to_owned(),
        "JSON" => type_str = "JSON.Value".to_owned(),
        _ => {
            if type_str.contains("Enum_") {
                type_str = type_str.replacen("Enum_", "", 1);
                is_enum = true;
            }
            type_str = detect_keyword_fn(&type_str);
            type_str = format!("Types.{}", type_str);
        }
    }

    apply_optional_fn(&type_str, optional, is_enum)
}

handlebars_helper!(to_wasm: |value: Value| {
    let value_str = value.as_str().unwrap();
    to_wasm_fn(value_str)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_to_wasm() {
        let res = to_wasm_fn("[[[[UInt32!]]!]]");
        assert_eq!(res, "Array<Array<Array<Array<u32> | null>> | null> | null");
    }
}
