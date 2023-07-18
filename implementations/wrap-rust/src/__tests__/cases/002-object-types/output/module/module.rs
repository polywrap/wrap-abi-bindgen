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
use crate::{
    ArgsMethod,
};
use crate::Arg;
use crate::Output;

pub struct Module;

pub trait ModuleTrait {
  fn method(args: ArgsMethod) -> Result<Output, String>;
}
