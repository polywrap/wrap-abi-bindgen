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
pub struct Env {
    pub prop: String,
    #[serde(rename = "optProp")]
    pub opt_prop: Option<String>,
    #[serde(rename = "optMap")]
    pub opt_map: Option<Map<String, Option<i32>>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            prop: String::new(),
            opt_prop: None,
            opt_map: None,
        }
    }
}
