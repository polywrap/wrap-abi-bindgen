lazy_static! {
  static ref NAME: String = "module.rs".to_string();
  static ref SOURCE: String = r#"/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, module::PluginModule};
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  JSON,
  bytes::ByteBuf,
  JSONString
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use super::types::*;

{{#with moduleType}}
{{#each methods}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Args{{to_upper name}} {
    {{#each arguments}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

{{/each}}
{{/with}}
pub trait Module: PluginModule {
  {{#with moduleType}}
  {{#each methods}}
  fn {{detect_keyword (to_lower name)}}(&mut self, args: &Args{{to_upper name}}, invoker: Arc<dyn Invoker>{{#with env}}, env: {{#if required}}{{else}}Option<{{/if}}Env{{#if required}}{{else}}>{{/if}}{{/with}}) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, PluginError>;
  {{#if (is_not_last @index ../methods)}}

  {{/if}}
  {{/each}}
  {{/with}}
}
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
