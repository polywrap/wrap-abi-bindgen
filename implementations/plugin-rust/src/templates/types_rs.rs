lazy_static! {
  static ref NAME: String = "types.rs".to_string();
  static ref SOURCE: String = r#"#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_core::{invoke::Invoker, uri::Uri};
use polywrap_plugin::error::PluginError;
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  BigInt,
  BigNumber,
  JSON,
  bytes,
  wrappers::{
    polywrap_bigint as bigint,
    polywrap_json as json
  }
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
{{#each importedModuleTypes}}
use std::sync::Arc;
{{/each}}

// Env START //

{{#with envType}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/with}}
// Env END //

// Objects START //

{{#each objectTypes}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/each}}
// Objects END //

// Enums START //

{{#each enumTypes}}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum {{detect_keyword (to_upper type)}} {
    {{#each constants}}
    {{detect_keyword this}},
    {{/each}}
    _MAX_
}
{{/each}}
// Enums END //

// Imported objects START //

{{#each importedObjectTypes}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/each}}
// Imported objects END //

// Imported envs START //

{{#each importedEnvType}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/each}}
// Imported envs END //

// Imported enums START //

{{#each importedEnumTypes}}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum {{detect_keyword (to_upper type)}} {
    {{#each constants}}
    {{detect_keyword this}},
    {{/each}}
    _MAX_
}
{{/each}}
// Imported enums END //

// Imported Modules START //

{{#each importedModuleTypes}}
{{#each methods}}
// URI: "{{../uri}}" //
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{to_upper ../type}}Args{{to_upper name}} {
    {{#each arguments}}
    {{#with scalar}}{{serde_annotate_if_bytes type}}{{/with}}{{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

{{/each}}
{{#if isInterface}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}}<'a> {
    uri: &'a str
}

impl<'a> {{detect_keyword (to_upper type)}}<'a> {
    pub const INTERFACE_URI: &'static str = "{{uri}}";

    pub fn new(uri: &'a str) -> {{detect_keyword (to_upper type)}}<'a> {
        {{detect_keyword (to_upper type)}} { uri: uri }
    }

    {{#each methods}}
    pub fn {{to_lower name}}(&self, args: &{{to_upper ../type}}Args{{to_upper name}}) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, PluginError> {
        let uri = self.uri;
        let serialized_args = to_vec(args.clone()).unwrap();
        let result = invoker.invoke_raw(
            uri,
            "{{name}}",
            serialized_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "{{name}}".to_string(),
            args: JSON::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok({{#with return}}{{#if required}}{{else}}Some({{/if}}{{/with}}from_slice(result.as_slice())?{{#with return}}{{#if required}}{{else}}){{/if}}{{/with}})
    }
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
}
{{else}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {}

impl {{detect_keyword (to_upper type)}} {
    pub const URI: &'static str = "{{uri}}";

    pub fn new() -> {{detect_keyword (to_upper type)}} {
        {{detect_keyword (to_upper type)}} {}
    }

    {{#each methods}}
    let uri = {{to_upper ../type}}::URI;
    pub fn {{detect_keyword (to_lower name)}}(args: &{{to_upper ../type}}Args{{to_upper name}}, invoker: Arc<dyn Invoker>) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, PluginError> {
        let serialized_args = to_vec(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let uri = Uri::try_from(uri).unwrap();
        let result = invoker.invoke_raw(
            &uri,
            "{{name}}",
            opt_args,
            None,
            None
        )
        .map_err(|e| PluginError::SubinvocationError {
            uri: uri.to_string(),
            method: "{{name}}".to_string(),
            args: JSON::to_string(&args).unwrap(),
            exception: e.to_string(),
        })?;

        Ok({{#with return}}{{#if required}}{{else}}Some({{/if}}{{/with}}from_slice(result.as_slice())?{{#with return}}{{#if required}}{{else}}){{/if}}{{/with}})
    }
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
}
{{/if}}
{{/each}}
// Imported Modules END //
"#.to_string();
}

use super::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
