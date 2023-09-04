use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    JSONString,
    BigIntWrapper,
    serde_bytes
};
use crate::Nested;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
    pub prop: String,
    pub nested: Nested,
}

impl Output {
    pub fn new() -> Output {
        Output {
            prop: String::new(),
            nested: Nested::new(),
        }
    }
}
