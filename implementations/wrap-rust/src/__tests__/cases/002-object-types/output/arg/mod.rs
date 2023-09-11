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
use crate::Nested;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Arg {
    pub prop: String,
    pub nested: Nested,
}

impl Arg {
    pub fn new() -> Arg {
        Arg {
            prop: String::new(),
            nested: Nested::new(),
        }
    }
}
