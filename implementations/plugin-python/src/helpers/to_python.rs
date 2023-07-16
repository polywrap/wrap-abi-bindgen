use handlebars::handlebars_helper;
use serde_json::value::Value;
use crate::helpers::detect_keyword::_detect_keyword;
use crate::helpers::to_upper::_to_upper;
use crate::helpers::util::{array_type, map_types};

handlebars_helper!(to_python: |value: Value| {
    _to_python(value.as_str().unwrap())
});

fn _to_python(value: &str) -> String {
    let mut type_val = value.to_string();

    let mut optional = false;
    if type_val.ends_with("!") {
        type_val.pop();
    } else {
        optional = true;
    }

    if type_val.starts_with("[") {
        return to_python_array(&type_val, optional).unwrap();
    }

    if type_val.starts_with("Map<") {
        return to_python_map(&type_val, optional).unwrap();
    }

    type_val = match type_val.as_str() {
        "Int" | "Int8" | "Int16" | "Int32" | "Int64" |
        "UInt" | "UInt8" | "UInt16" | "UInt32" | "UInt64" => "int".to_string(),
        "String" | "BigInt" | "BigNumber" | "JSON" => "str".to_string(),
        "Boolean" => "bool".to_string(),
        "Bytes" => "bytes".to_string(),
        _ => {
            if type_val.starts_with("Enum_") {
                type_val = type_val.replacen("Enum_", "", 1);
            }
            type_val = _to_upper(&type_val);
            type_val = _detect_keyword(&type_val);
            format!("\"{}\"", type_val)
        }
    };

    apply_optional(&type_val, optional)
}

fn to_python_array(value: &str, optional: bool) -> Result<String, String> {
    let inner_type = array_type(value)?;
    let py_type = _to_python(&inner_type);
    let py_array = format!("list[{}]", py_type);
    Ok(apply_optional(&py_array, optional))
}

fn to_python_map(value: &str, optional: bool) -> Result<String, String> {
    let (key_type, val_type) = map_types(value)?;
    let py_key_type = _to_python(&key_type);
    let py_val_type = _to_python(&val_type);
    let py_map = format!("GenericMap[{}, {}]", py_key_type, py_val_type);
    Ok(apply_optional(&py_map, optional))
}

fn apply_optional(value: &str, optional: bool) -> String {
    if optional {
        format!("Optional[{}]", value)
    } else {
        value.to_string()
    }
}

