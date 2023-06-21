use polywrap_wasm_rs::{
  wrap_load_env
};

use crate::{
    ArgsFunction1,
    deserialize_function1_args,
    serialize_function1_result,
    ArgsFunction2,
    deserialize_function2_args,
    serialize_function2_result
};

use crate::module::{ModuleTrait, Module};

pub fn function1_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_function1_args(args) {
        Ok(args) => {
            let result = Module::function1(ArgsFunction1 {
                arg1: args.arg1,
                arg2: args.arg2,
            });
            match result {
                Ok(res) => {
                    serialize_function1_result(&res).unwrap()
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

pub fn function2_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_function2_args(args) {
        Ok(args) => {
            let result = Module::function2(ArgsFunction2 {
                arg1: args.arg1,
                arg2: args.arg2,
            });
            match result {
                Ok(res) => {
                    serialize_function2_result(&res).unwrap()
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
