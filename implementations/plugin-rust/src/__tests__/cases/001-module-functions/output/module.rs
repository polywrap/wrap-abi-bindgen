/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoke::Invoker;
use polywrap_plugin::{error::PluginError, module::PluginModule};
use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsFunction1 {
    pub arg1: u32,
    pub arg2: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsFunction2 {
    pub arg1: Option<i32>,
    #[serde(with = "serde_bytes")]
    pub arg2: Option<Vec<u8>>,
}

pub trait Module: PluginModule {
  fn function1(&mut self, args: &ArgsFunction1, invoker: Arc<dyn Invoker>) -> Result<String, PluginError>;

  fn function2(&mut self, args: &ArgsFunction2, invoker: Arc<dyn Invoker>) -> Result<Option<String>, PluginError>;
}
