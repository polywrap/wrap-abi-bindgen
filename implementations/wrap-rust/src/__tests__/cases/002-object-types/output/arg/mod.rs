use serde::{Serialize, Deserialize};
use polywrap_msgpack_serde::{
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON
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
