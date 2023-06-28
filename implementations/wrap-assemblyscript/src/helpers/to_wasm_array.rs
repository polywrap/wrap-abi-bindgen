use handlebars::handlebars_helper;
use serde_json::{Value};

use super::apply_optional::apply_optional_fn;
use super::to_wasm::to_wasm_fn;

pub fn to_wasm_array_fn(value: &str, optional: bool) -> Result<String, String> {
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
    let wasm_type = to_wasm_fn(inner_type);

    let wasm_array = format!("Array<{}>", wasm_type);

    Ok(apply_optional_fn(&wasm_array, optional, false))
}

handlebars_helper!(to_wasm_array: |value: Value, optional: bool| {
    let value_str = value.as_str().unwrap();
    to_wasm_array_fn(value_str, optional).unwrap()
});
