use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::detect_keyword::_detect_keyword;
use crate::helpers::to_upper::_to_upper;
use crate::helpers::util::{array_type, map_types};

handlebars_helper!(to_rust: |val: Value| {
    let type_val = val.as_str().unwrap();
    _to_rust(type_val)
});

pub fn _to_rust(value: &str) -> String {
    let mut res = value.to_string();
    let mut optional = false;
    if res.ends_with("!") {
        res.pop();
    } else {
        optional = true;
    }

    if res.starts_with("[") {
        return _to_rust_array(&res, optional).unwrap();
    }

    if res.starts_with("Map<") {
        return _to_rust_map(&res, optional).unwrap();
    }

    res = match res.as_str() {
        "Int" | "Int32" => "i32".to_string(),
        "Int8" => "i8".to_string(),
        "Int16" => "i16".to_string(),
        "Int64" => "i64".to_string(),
        "UInt" | "UInt32" => "u32".to_string(),
        "UInt8" => "u8".to_string(),
        "UInt16" => "u16".to_string(),
        "UInt64" => "u64".to_string(),
        "String" => "String".to_string(),
        "Boolean" => "bool".to_string(),
        "Bytes" => "ByteBuf".to_string(),
        "BigInt" => "BigInt".to_string(),
        "BigNumber" => "BigNumber".to_string(),
        "JSON" => "JSONString".to_string(),
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

pub fn _to_rust_array(value: &str, optional: bool) -> Result<String, String> {
    let inner_type = array_type(value)?;
    let rs_type = _to_rust(&inner_type);
    let rs_array = format!("Vec<{}>", rs_type);
    Ok(_apply_optional(&rs_array, optional))
}

pub fn _to_rust_map(value: &str, optional: bool) -> Result<String, String> {
    let (key_type, val_type) = map_types(value)?;
    let rs_key_type = _to_rust(&key_type);
    let rs_val_type = _to_rust(&val_type);
    let rs_map = format!("BTreeMap<{}, {}>", &rs_key_type, &rs_val_type);
    Ok(_apply_optional(&rs_map, optional))
}

pub fn _apply_optional(value: &str, optional: bool) -> String {
    return if optional {
        format!("Option<{}>", value)
    } else {
        value.to_string()
    }
}