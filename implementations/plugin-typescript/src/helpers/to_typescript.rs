use handlebars::handlebars_helper;
use serde_json::value::Value;
use crate::helpers::detect_keyword::_detect_keyword;

handlebars_helper!(to_typescript: |value: Value| {
    _to_typescript(value.as_str().unwrap(), false)
});

fn _to_typescript(value: &str, undefinable: bool) -> String {
    let mut type_val = value.to_string();

    let mut optional = false;
    if type_val.chars().last().unwrap() == '!' {
        type_val.pop();
    } else {
        optional = true;
    }

    if type_val.starts_with("[") {
        return to_typescript_array(&type_val, optional).unwrap();
    }

    if type_val.starts_with("Map<") {
        return to_typescript_map(&type_val, optional).unwrap();
    }

    match type_val.as_str() {
        "JSON" => type_val = "Types.Json".to_string(),
        _ => {
            if type_val.starts_with("Enum_") {
                type_val = type_val.replacen("Enum_", "", 1);
            }
            type_val = _detect_keyword(&type_val);
            type_val = format!("Types.{}", type_val)
        }
    }

    return if undefinable {
        apply_undefinable(&type_val, optional)
    } else {
        apply_optional(&type_val, optional)
    }
}

fn to_typescript_array(value: &str, optional: bool) -> Result<String, String> {
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
    let ts_type = _to_typescript(inner_type, false);

    let ts_array = format!("Array<{}>", ts_type);
    Ok(apply_optional(&ts_array, optional))
}

fn to_typescript_map(value: &str, optional: bool) -> Result<String, String> {
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

    let ts_key_type = _to_typescript(key_type, false);
    let ts_val_type = _to_typescript(val_type, true);

    let ts_map = format!("Map<{}, {}>", ts_key_type, ts_val_type);
    Ok(apply_optional(&ts_map, optional))
}

fn apply_optional(type_val: &str, optional: bool) -> String {
    if optional {
        format!("{} | null", type_val)
    } else {
        type_val.to_string()
    }
}

fn apply_undefinable(type_val: &str, undefinable: bool) -> String {
    if undefinable {
        format!("{} | undefined", type_val)
    } else {
        type_val.to_string()
    }
}


