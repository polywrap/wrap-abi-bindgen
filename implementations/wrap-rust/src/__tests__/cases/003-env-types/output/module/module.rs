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
    ArgsMethodNoEnv,
    ArgsMethodRequireEnv,
    ArgsMethodOptionalEnv,
};
use crate::Env;

pub struct Module;

pub trait ModuleTrait {
  fn method_no_env(args: ArgsMethodNoEnv) -> Result<String, String>;

  fn method_require_env(args: ArgsMethodRequireEnv, env: Env) -> Result<Env, String>;

  fn method_optional_env(args: ArgsMethodOptionalEnv, env: Option<Env>) -> Result<Option<Env>, String>;
}
