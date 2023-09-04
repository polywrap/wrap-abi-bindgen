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
use crate::{
    ArgsMethod,
};
use crate::Arg;
use crate::Output;

pub struct Module;

pub trait ModuleTrait {
  fn method(args: ArgsMethod) -> Result<Output, String>;
}
