pub fn replace_at(s: &str, idx: usize, replacement: &str) -> String {
    let start = s[..idx].to_string();
    let end = s[idx + replacement.len()..].to_string();
    format!("{}{}{}", start, replacement, end)
}

pub fn insert_at(s: &str, idx: usize, insert: &str) -> String {
    let start = s[..idx].to_string();
    let end = s[idx..].to_string();
    format!("{}{}{}", start, insert, end)
}

pub fn remove_at(s: &str, idx: usize) -> String {
    let start = s[..idx].to_string();
    let end = s[idx + 1..].to_string();
    format!("{}{}", start, end)
}

pub fn array_type(value: &str) -> Result<String, String> {
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
    Ok(inner_type.to_string())
}

pub fn map_types(value: &str) -> Result<(String, String), String> {
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

    Ok((key_type.to_string(), val_type.to_string()))
}