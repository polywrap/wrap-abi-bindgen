use handlebars::handlebars_helper;
use serde_json::Value;

use super::to_wasm_array::to_wasm_array_fn;
use super::to_wasm_map::to_wasm_map_fn;
use super::detect_keyword::detect_keyword_fn;
use super::apply_optional::apply_optional_fn;

pub fn to_wasm_fn(value: &str, skip_types_prefix: Option<bool>) -> String {
    let mut type_str = String::from(value);
    let mut is_enum = false;
    let mut optional = false;

    if type_str.ends_with('!') {
        type_str = type_str.trim_end_matches('!').to_string();
    } else {
        optional = true;
    }

    if type_str.starts_with('[') {
        return to_wasm_array_fn(&type_str, optional, skip_types_prefix).unwrap();
    }

    if type_str.starts_with("Map<") {
        return to_wasm_map_fn(&type_str, optional, skip_types_prefix).unwrap();
    }

    match type_str.as_str() {
        "Int" => type_str = "number".to_owned(),
        "Int8" => type_str = "number".to_owned(),
        "Int16" => type_str = "number".to_owned(),
        "Int32" => type_str = "number".to_owned(),
        "UInt" | "UInt32" => type_str = "number".to_owned(),
        "UInt8" => type_str = "number".to_owned(),
        "UInt16" => type_str = "number".to_owned(),
        "String" => type_str = "string".to_owned(),
        "Boolean" => type_str = "boolean".to_owned(),
        "Bytes" => type_str = "ArrayBuffer".to_owned(),
        "BigInt" => type_str = "BigInt".to_owned(),
        "BigNumber" => type_str = "BigNumber".to_owned(),
        "JSON" => type_str = "JSONString".to_owned(),
        _ => {
            if type_str.starts_with("Enum_") {
                type_str = type_str.replacen("Enum_", "", 1);
                is_enum = true;
            }
            type_str = detect_keyword_fn(&type_str);
            type_str = if skip_types_prefix.is_some() && skip_types_prefix.unwrap() {
              format!("{}", type_str)
            } else {
              format!("Types.{}", type_str)
            }
        }
    }

    apply_optional_fn(&type_str, optional, is_enum)
}

handlebars_helper!(to_wasm: |value: Value, skip_types_prefix: Option<Value>| {
    let value_str = value.as_str().unwrap();
    let skip_types_prefix_bool = if let Some(skip_types_prefix) = skip_types_prefix {
      Some(skip_types_prefix.as_bool().unwrap())
    } else {
      None
    };
    to_wasm_fn(value_str, skip_types_prefix_bool)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_to_wasm() {
        let res = to_wasm_fn("[[[[UInt32!]]!]]", None);
        assert_eq!(res, "Array<Array<Array<Array<number> | null>> | null> | null");
    }
}
