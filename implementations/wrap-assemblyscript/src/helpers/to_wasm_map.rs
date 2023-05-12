use handlebars::handlebars_helper;

use super::apply_optional::apply_optional_fn;
use super::to_wasm::to_wasm_fn;

pub fn to_wasm_map_fn(type_str: &String, optional: bool) -> Result<String, String> {
    let first_open_bracket_idx = type_str.find('<').unwrap_or(0);
    let last_close_bracket_idx = type_str.rfind('>').unwrap_or(0);

    if first_open_bracket_idx == 0 || last_close_bracket_idx == 0 {
        return Err(format!("Invalid Map: {}", type_str));
    }

    let key_val_types = type_str[first_open_bracket_idx + 1..last_close_bracket_idx].trim();

    let first_comma_idx = key_val_types.find(',').unwrap_or(0);
    if first_comma_idx == 0 {
        return Err(format!("Invalid Map: {}", type_str));
    }

    let key_type = key_val_types[..first_comma_idx].trim();
    let val_type = key_val_types[first_comma_idx + 1..].trim();

    let wasm_key_type = to_wasm_fn(&key_type.to_string());
    let wasm_val_type = to_wasm_fn(&val_type.to_string());

    Ok(apply_optional_fn(&format!("Map<{}, {}>", wasm_key_type, wasm_val_type), optional, false))
}

handlebars_helper!(to_wasm_map: |value: str, optional: bool| {
    to_wasm_map_fn(&value.to_string(), optional).unwrap()
});
