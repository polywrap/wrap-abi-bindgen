use polywrap_wasm_rs::{
  BigInt,
  BigNumber,
  Map,
  JSON,
  JSONString,
  BigIntWrapper,
  ByteBuf
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
