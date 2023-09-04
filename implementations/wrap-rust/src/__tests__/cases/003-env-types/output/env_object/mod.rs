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
pub struct EnvObject {
    pub prop: String,
}

impl EnvObject {
    pub fn new() -> EnvObject {
        EnvObject {
            prop: String::new(),
        }
    }
}
