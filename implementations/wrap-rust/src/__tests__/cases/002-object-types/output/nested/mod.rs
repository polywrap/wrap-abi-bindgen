use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env,
    to_vec,
    from_slice,
    JSONString,
    BigIntWrapper
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
