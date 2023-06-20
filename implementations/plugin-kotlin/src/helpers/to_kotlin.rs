use handlebars::handlebars_helper;
use serde_json::value::Value;
use crate::helpers::detect_keyword::_detect_keyword;
use crate::helpers::to_upper::_to_upper;

handlebars_helper!(to_kotlin: |value: Value| {
    _to_kotlin(value.as_str().unwrap())
});

fn _to_kotlin(value: &str) -> String {
    let mut type_val = value.to_string();

    let mut optional = false;
    if type_val.ends_with("!") {
        type_val.pop();
    } else {
        optional = true;
    }

    if type_val.starts_with("[") {
        return to_kotlin_array(&type_val, optional).unwrap();
    }

    if type_val.starts_with("Map<") {
        return to_kotlin_map(&type_val, optional).unwrap();
    }

    type_val = match type_val.as_str() {
        "Int" | "Int32" => "Int".to_string(),
        "Int8" => "Byte".to_string(),
        "Int16" => "Short".to_string(),
        "Int64" => "Long".to_string(),
        "UInt" | "UInt32" => "UInt".to_string(),
        "UInt8" => "UByte".to_string(),
        "UInt16" => "UShort".to_string(),
        "UInt64" => "ULong".to_string(),
        "String" => "String".to_string(),
        "Boolean" => "Boolean".to_string(),
        "Bytes" => "ByteArray".to_string(),
        "BigInt" => "BigInt".to_string(),
        "BigNumber" => "BigNumber".to_string(),
        "JSON" => "Json".to_string(),
        _ => {
            if type_val.starts_with("Enum_") {
                type_val = type_val.replacen("Enum_", "", 1);
            }
            type_val = _to_upper(&type_val);
            _detect_keyword(&type_val)
        }
    };

    apply_optional(&type_val, optional)
}

fn to_kotlin_array(value: &str, optional: bool) -> Result<String, String> {
    let mut iter = value.char_indices();

    let first_bracket = match iter.find(|&(_, c)| c == '[').map(|(i, _)| i) {
        Some(idx) => idx,
        None => return Err(format!("Invalid Array: {}", value)),
    };
    let last_bracket = match iter.rfind(|&(_, c)| c == ']').map(|(i, _)| i) {
        Some(idx) => idx,
        None => return Err(format!("Invalid Array: {}", value)),
    };

    let inner_type = &value[(first_bracket+1)..last_bracket];
    let kt_type = _to_kotlin(inner_type);

    let kt_array = format!("List<{}>", kt_type);
    Ok(apply_optional(&kt_array, optional))
}

fn to_kotlin_map(value: &str, optional: bool) -> Result<String, String> {
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
    let val_type = key_val_types[(first_comma_idx+1)..].trim();

    let kt_key_type = _to_kotlin(key_type);
    let kt_val_type = _to_kotlin(val_type);

    let kt_map = format!("MsgPackMap<{}, {}>", kt_key_type, kt_val_type);
    Ok(apply_optional(&kt_map, optional))
}

fn apply_optional(value: &str, optional: bool) -> String {
    if optional {
        format!("{}?", value)
    } else {
        value.to_string()
    }
}




