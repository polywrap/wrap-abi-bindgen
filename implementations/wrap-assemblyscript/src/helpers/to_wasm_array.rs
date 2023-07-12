use handlebars::handlebars_helper;
use serde_json::{Value};
use crate::helpers::util::array_type;

use super::apply_optional::apply_optional_fn;
use super::to_wasm::to_wasm_fn;

pub fn to_wasm_array_fn(value: &str, optional: bool) -> Result<String, String> {
    let inner_type = array_type(value)?;
    let wasm_type = to_wasm_fn(&inner_type);
    let wasm_array = format!("Array<{}>", wasm_type);
    Ok(apply_optional_fn(&wasm_array, optional, false))
}

handlebars_helper!(to_wasm_array: |value: Value, optional: bool| {
    let value_str = value.as_str().unwrap();
    to_wasm_array_fn(value_str, optional).unwrap()
});
