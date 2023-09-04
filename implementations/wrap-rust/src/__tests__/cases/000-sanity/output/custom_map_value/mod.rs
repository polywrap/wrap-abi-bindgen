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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomMapValue {
    pub foo: String,
}

impl CustomMapValue {
    pub fn new() -> CustomMapValue {
        CustomMapValue {
            foo: String::new(),
        }
    }
}
