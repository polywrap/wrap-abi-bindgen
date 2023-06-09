use serde::{Deserialize, Serialize};
use polywrap_msgpack_serde::{
    from_slice,
    to_vec,
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env
};
use crate::module::{ModuleTrait, Module};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsFunction1 {
    pub arg1: u32,
    pub arg2: bool,
}

pub fn function1_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsFunction1>(args) {
        Ok(args) => {
            let result = Module::function1(ArgsFunction1 {
                arg1: args.arg1,
                arg2: args.arg2,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsFunction2 {
    pub arg1: Option<i32>,
    #[serde(with = "serde_bytes")]
    pub arg2: Option<Vec<u8>>,
}

pub fn function2_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsFunction2>(args) {
        Ok(args) => {
            let result = Module::function2(ArgsFunction2 {
                arg1: args.arg1,
                arg2: args.arg2,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
