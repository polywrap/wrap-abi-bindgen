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
        return to_typescript_array(&type_val, optional);
    }

    if type_val.starts_with("Map<") {
        return to_typescript_map(&type_val, optional);
    }

    match type_val.as_str() {
        "JSON" => type_val = "Types.Json".to_string(),
        _ => {
            if type_val.contains("Enum_") {
                type_val = type_val.replace("Enum_", "");
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

fn to_typescript_array(type_val: &str, optional: bool) -> String {
    let mut iter = type_val.char_indices();

    let first_bracket = iter.find(|&(_, c)| c == '[').map(|(i, _)| i).unwrap();
    let last_bracket = iter.rfind(|&(_, c)| c == ']').map(|(i, _)| i).unwrap();

    let inner_type = &type_val[(first_bracket+1)..last_bracket];
    let ts_type = _to_typescript(inner_type, false);

    apply_optional(&format!("Array<{}>", ts_type), optional)
}

fn to_typescript_map(type_val: &str, optional: bool) -> String {
    let open_angle_bracket_idx = type_val.find('<').unwrap();
    let close_angle_bracket_idx = type_val.rfind('>').unwrap();

    let key_val_types = &type_val[(open_angle_bracket_idx+1)..close_angle_bracket_idx];

    let first_comma_idx = key_val_types.find(',').unwrap();
    let key_type = key_val_types[..first_comma_idx].trim();
    let val_type = key_val_types[(first_comma_idx+1)..].trim();

    let ts_key_type = _to_typescript(key_type, false);
    let ts_val_type = _to_typescript(val_type, true);

    apply_optional(&format!("Map<{}, {}>", ts_key_type, ts_val_type), optional)
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

fn first_upper(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn first_lower(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().chain(c).collect(),
    }
}

