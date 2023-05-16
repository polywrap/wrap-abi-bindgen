use regex::Regex;
use handlebars::handlebars_helper;
use serde_json::{Value};

use super::apply_optional::apply_optional_fn;
use super::to_wasm::to_wasm_fn;

pub fn to_wasm_array_fn(value: &str, optional: bool) -> Result<String, String> {
    let re = Regex::new(r"(\[)([\[\]A-Za-z0-9_.!]+)(\])").unwrap();
    let captures = re.captures(value);

    let result = match captures {
        Some(captures) => captures,
        None => return Err(format!("Invalid Array: {}", value)),
    };

    let wasm_type = to_wasm_fn(&result.get(2).unwrap().as_str().to_string());
    Ok(apply_optional_fn(&format!("Array<{}>", wasm_type), optional, false))
}

handlebars_helper!(to_wasm_array: |value: Value, optional: bool| {
    let value_str = value.as_str().unwrap();
    to_wasm_array_fn(value_str, optional).unwrap()
});
