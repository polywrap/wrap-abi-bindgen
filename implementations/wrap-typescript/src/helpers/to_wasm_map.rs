use handlebars::handlebars_helper;
use serde_json::Value;
use crate::helpers::util::map_types;

use super::apply_optional::apply_optional_fn;
use super::to_wasm::to_wasm_fn;

pub fn to_wasm_map_fn(value: &str, optional: bool, skip_types_prefix: Option<bool>) -> Result<String, String> {
    let (key_type, val_type) = map_types(value)?;
    let wasm_key_type = to_wasm_fn(&key_type, skip_types_prefix);
    let wasm_val_type = to_wasm_fn(&val_type, skip_types_prefix);
    let wasm_map = format!("Map<{}, {}>", wasm_key_type, wasm_val_type);
    Ok(apply_optional_fn(&wasm_map, optional, false))
}

handlebars_helper!(to_wasm_map: |value: Value, optional: bool, skip_types_prefix: Option<Value>| {
    let value_str = value.as_str().unwrap();
    let skip_types_prefix_bool = if let Some(skip_types_prefix) = skip_types_prefix {
      Some(skip_types_prefix.as_bool().unwrap())
    } else {
      None
    };
    to_wasm_map_fn(value_str, optional, skip_types_prefix_bool).unwrap()
});
