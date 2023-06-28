use handlebars::handlebars_helper;
use serde_json::{Value};

use super::apply_optional::apply_optional_fn;
use super::to_wasm::to_wasm_fn;

pub fn to_wasm_map_fn(value: &str, optional: bool) -> Result<String, String> {
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

    let wasm_key_type = to_wasm_fn(&key_type.to_string());
    let wasm_val_type = to_wasm_fn(&val_type.to_string());

    let wasm_map = format!("Map<{}, {}>", wasm_key_type, wasm_val_type);
    Ok(apply_optional_fn(&wasm_map, optional, false))
}

handlebars_helper!(to_wasm_map: |value: Value, optional: bool| {
    let value_str = value.as_str().unwrap();
    to_wasm_map_fn(value_str, optional).unwrap()
});
