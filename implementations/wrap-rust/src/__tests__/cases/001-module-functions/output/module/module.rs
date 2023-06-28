use crate::{
    ArgsFunction1,
    ArgsFunction2,
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
};

pub struct Module;

pub trait ModuleTrait {
  fn function1(args: ArgsFunction1) -> Result<String, String>;

  fn function2(args: ArgsFunction2) -> Result<Option<String>, String>;
}
