use handlebars::handlebars_helper;
use serde_json::{Value};
use regex::Regex;
use crate::helpers::detect_keyword::_detect_keyword;
use crate::helpers::to_upper::_to_upper;

handlebars_helper!(to_wasm: |val: Value| {
    let type_val = val.as_str().unwrap();
    _to_wasm(type_val)
});

pub fn _to_wasm(value: &str) -> String {
    let mut res = value.to_string();
    let mut optional = false;
    if res.ends_with("!") {
        res.pop();
    } else {
        optional = true;
    }

    if res.starts_with("[") {
        return _to_wasm_array(&res, optional).unwrap();
    }

    if res.starts_with("Map<") {
        return _to_wasm_map(&res, optional).unwrap();
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
        "Bytes" => "Vec<u8>".to_string(),
        "BigInt" => "BigInt".to_string(),
        "BigNumber" => "BigNumber".to_string(),
        "JSON" => "JSON::Value".to_string(),
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

pub fn _to_wasm_array(value: &str, optional: bool) -> Result<String, String> {
    let maybe_captures = match Regex::new(r"(\[)([\[\]A-Za-z0-9_.!]+)(\])") {
        Ok(re) => re.captures(value),
        Err(err) => return Err(format!("Error while compiling Regex: {}", err.to_string())),
    };

    let captures = match maybe_captures {
        Some(caps) => caps,
        None => return Err(format!("Invalid Array: {}", value)),
    };

    if captures.len() != 4 {
        return Err(format!("Invalid Array: {}", value));
    }

    match captures.get(2) {
        Some(match_value) => {
            let wasm_type = _to_wasm(match_value.as_str());
            let wasm_array = format!("Vec<{}>", wasm_type);
            Ok(_apply_optional(&wasm_array, optional))
        },
        None => Err(format!("Invalid Array: {}", value)),
    }
}

pub fn _to_wasm_map(value: &str, optional: bool) -> Result<String, String> {
    let first_open_bracket_idx = match value.find('<') {
        Some(idx) => idx,
        None => return Err(format!("Invalid Map: {}", value)),
    };
    let last_close_bracket_idx = match value.rfind('>') {
        Some(idx) => idx,
        None => return Err(format!("Invalid Map: {}", value)),
    };

    let key_val_types = &value[(first_open_bracket_idx + 1)..last_close_bracket_idx];

    let first_comma_idx = match key_val_types.find(',') {
        Some(idx) => idx,
        None => return Err(format!("Invalid Map: {}", value)),
    };

    let key_type = key_val_types[..first_comma_idx].trim();
    let val_type = key_val_types[(first_comma_idx + 1)..].trim();

    let wasm_key_type = _to_wasm(key_type);
    let wasm_val_type = _to_wasm(val_type);

    let wasm_map = format!("Map<{}, {}>", wasm_key_type, wasm_val_type);
    Ok(_apply_optional(&wasm_map, optional))
}

pub fn _apply_optional(value: &str, optional: bool) -> String {
    return if optional {
        format!("Option<{}>", value)
    } else {
        value.to_string()
    }
}