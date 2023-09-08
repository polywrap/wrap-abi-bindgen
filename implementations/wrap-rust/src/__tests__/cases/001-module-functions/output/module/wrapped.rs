use serde::{Deserialize, Serialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env,
    to_vec,
    from_slice,
    JSONString,
    BigIntWrapper,
    ByteBuf
};
use crate::{ModuleTrait, Module};

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
    pub arg2: Option<ByteBuf>,
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
