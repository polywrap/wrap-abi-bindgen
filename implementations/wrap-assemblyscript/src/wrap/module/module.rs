use crate::{
    ArgsGenerateBindings,
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
};

use crate::WrapAbi;
use crate::Output;

pub struct Module;

pub trait ModuleTrait {
  fn generate_bindings(args: ArgsGenerateBindings) -> Result<Output, String>;
}
