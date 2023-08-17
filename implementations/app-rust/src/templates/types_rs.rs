lazy_static! {
  static ref NAME: String = "types.rs".to_string();
  static ref SOURCE: String = r#"#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap_core::{invoker::Invoker, uri::Uri};
use polywrap_plugin::error::PluginError;
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  JSON,
  bytes::ByteBuf,
  JSONString,
  BigNumber
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
{{#each importedModuleTypes}}
use std::sync::Arc;
{{/each}}

pub type BigInt = String;

// Env START //

{{#with envType}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/with}}
// Env END //

// Objects START //

{{#each objectTypes}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
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
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/each}}
// Imported objects END //

// Imported envs START //

{{#each importedEnvType}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
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
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

{{/each}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{to_abstract_type (detect_keyword (to_upper type))}} {
    uri: Uri,
    invoker: Arc<dyn Invoker>,
    env: Option<Vec<u8>>
}

impl {{detect_keyword (to_upper type)}} {
    pub const URI: &'static str = "{{uri}}";

    pub fn new(uri: Option<Uri>, invoker: Arc<dyn Invoker>, env: Option<Vec<u8>>) -> {{detect_keyword (to_upper type)}} {
        let _uri = uri.unwrap_or(Uri::try_from(Self::URI).unwrap());
        let _invoker = invoker.unwrap();
        let _env = env;

        {{detect_keyword (to_upper type)}} {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    {{#each methods}}
    pub fn {{detect_keyword (to_lower name)}}(&self, args: &{{to_upper ../type}}Args{{to_upper name}}, uri: Option<Uri>, invoker: Option<Arc<dyn Invoker>>, env: Option<Vec<u8>>) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, PluginError> {
        let _uri = uri.unwrap_or(self.uri.clone());
        let _invoker = invoker.unwrap_or(self.invoker.clone());
        let _env = match env {
            Some(e) => Some(e),
            None => self.env.clone(),
        };

        let serialized_args = to_vec(args.clone()).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "{{name}}",
            opt_args,
            _env,
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
