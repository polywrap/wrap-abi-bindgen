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
    serde_bytes
};
use crate::module::{ModuleTrait, Module};
use crate::Env;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsMethodNoEnv {
    pub arg: String,
}

pub fn method_no_env_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsMethodNoEnv>(args) {
        Ok(args) => {
            let result = Module::method_no_env(ArgsMethodNoEnv {
                arg: args.arg,
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
pub struct ArgsMethodRequireEnv {
}

pub fn method_require_env_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    if env_size == 0 {
        panic!("Environment is not set, and it is required by method 'methodRequireEnv'");
    }

    let env_buf = wrap_load_env(env_size);
    let env = from_slice::<Env>(&env_buf).unwrap();

            let result = Module::method_require_env(ArgsMethodRequireEnv {
            }, env);
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsMethodOptionalEnv {
}

pub fn method_optional_env_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    let mut env: Option<Env> = None;
    if env_size > 0 {
      let env_buf = wrap_load_env(env_size);
      env = Some(from_slice::<Env>(&env_buf).unwrap());
    }

            let result = Module::method_optional_env(ArgsMethodOptionalEnv {
            }, env);
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
}
