use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::detect_keyword::_detect_keyword;
use crate::helpers::to_upper::_to_upper;
use crate::helpers::util::{array_type, map_types};

handlebars_helper!(to_rust: |val: Value| {
    let type_val = val.as_str().unwrap();
    to_swift(type_val)
});

pub fn to_swift(value: &str) -> String {
    let mut res = value.to_string();
    let mut optional = false;
    if res.ends_with("!") {
        res.pop();
    } else {
        optional = true;
    }

    if res.starts_with("[") {
        return to_swift_array(&res, optional).unwrap();
    }

    if res.starts_with("Map<") {
        return to_swift_map(&res, optional).unwrap();
    }

    res = match res.as_str() {
        "Int" | "Int32" => "Int32".to_string(),
        "Int8" => "Int8".to_string(),
        "Int16" => "Int16".to_string(),
        "Int64" => "Int64".to_string(),
        "UInt" | "UInt32" => "UInt32".to_string(),
        "UInt8" => "UInt8".to_string(),
        "UInt16" => "UInt16".to_string(),
        "UInt64" => "UInt64".to_string(),
        "Boolean" => "Bool".to_string(),
        "Bytes" => "Data".to_string(),
        "String" | "BigInt" | "BigNumber" | "JSON" => "String".to_string(),
        _ => {
            if res.starts_with("Enum_") {
                res = res.replacen("Enum_", "", 1);
            }
            res = _to_upper(&res);
            _detect_keyword(&res)
        }
    };

    _apply_optional(&res, optional)
}

pub fn _to_swift_array(value: &str, optional: bool) -> Result<String, String> {
    let inner_type = array_type(value)?;
    let swift_type = to_swift(&inner_type);
    let rs_array = format!("Array<{}>", swift_type);
    Ok(_apply_optional(&rs_array, optional))
}

pub fn to_swift_map(value: &str, optional: bool) -> Result<String, String> {
    let (key_type, val_type) = map_types(value)?;
    let swift_key_type = to_swift(&key_type);
    let swift_val_type = to_swift(&val_type);
    let swift_map = format!("[{}: {}]", &swift_key_type, &swift_val_type);
    Ok(_apply_optional(&swift_map, optional))
}

pub fn _apply_optional(value: &str, optional: bool) -> String {
    return if optional {
        format!("Option<{}>", value)
    } else {
        value.to_string()
    }
}