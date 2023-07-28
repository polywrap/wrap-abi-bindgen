use handlebars::handlebars_helper;
use serde_json::{Value};
use polywrap_msgpack::serialize;

handlebars_helper!(to_kotlin_byte_array: |abi: Value| {
    _to_kotlin_byte_array(abi)
});

pub fn _to_kotlin_byte_array(abi: Value) -> String {
    let bytes: Value = serialize(&abi).unwrap()
        .iter()
        .map(|x| format!("{}.toByte()", x))
        .collect::<Vec<String>>()
        .into();
    let bytes_str: String = bytes.to_string()
        .replace("\"", "")
        .replace("\\", "")
        .trim_start_matches("[")
        .trim_end_matches("]")
        .trim()
        .to_string();
    format!("byteArrayOf(\n        {}\n        )", bytes_str)
}
