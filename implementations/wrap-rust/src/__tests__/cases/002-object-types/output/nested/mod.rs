use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    JSONString,
    BigIntWrapper,
    ByteBuf
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Nested {
    pub prop: String,
}

impl Nested {
    pub fn new() -> Nested {
        Nested {
            prop: String::new(),
        }
    }
}
