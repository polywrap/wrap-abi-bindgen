lazy_static! {
  static ref NAME: String = "types.rs".to_string();
  static ref SOURCE: String = r#"#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// NOTE: This is an auto-generated file.
//       All modifications will be overwritten.
use polywrap::*;
use std::result::Result;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

pub type BigInt = String;

#[derive(Clone)]
pub struct InvokeOptions {
    pub uri: Option<Uri>,
    pub client: Option<Arc<dyn Invoker>>,
    pub env: Option<Vec<u8>> 
}

fn get_default_client() -> Arc<PolywrapClient> {
    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());
    config.add(Web3ClientConfig::default().into());
    let client = PolywrapClient::new(config.build());
    Arc::new(client)
}

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
pub struct {{remove_module_suffix (detect_keyword (to_upper type))}} {
    {{#each properties}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/each}}
// Objects END //

// Enums START //

{{#each enumTypes}}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum {{remove_module_suffix (detect_keyword (to_upper type))}} {
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
pub struct {{remove_module_suffix (detect_keyword (to_upper type))}} {
    {{#each properties}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/each}}
// Imported objects END //

// Imported envs START //

{{#each importedEnvTypes}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{remove_module_suffix (detect_keyword (to_upper type))}} {
    {{#each properties}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{/each}}
// Imported envs END //

// Imported enums START //

{{#each importedEnumTypes}}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum {{remove_module_suffix (detect_keyword (to_upper type))}} {
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
pub struct {{remove_module_suffix (to_upper ../type) }}Args{{to_upper name}} {
    {{#each arguments}}
    {{serde_rename_if_case_mismatch name}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

{{/each}}
#[derive(Clone)]
pub struct {{remove_module_suffix (detect_keyword (to_upper type))}} {
    pub uri: Uri,
    pub invoker: Arc<dyn Invoker>,
    pub env: Option<Vec<u8>>
}

impl {{remove_module_suffix (detect_keyword (to_upper type))}} {
    pub fn new(invoke_options: Option<InvokeOptions>) -> {{remove_module_suffix (detect_keyword (to_upper type))}} {
        let default_uri = uri!("{{uri}}");
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let _uri = if let Some(uri) = invoke_option.uri {
                uri
            } else {
                default_uri
            };

            let _invoker = if let Some(invoker) = invoke_option.client {
                invoker
            } else {
                get_default_client()
            };

            (_uri, _invoker, invoke_option.env)
        } else {
            (default_uri, get_default_client() as Arc<dyn Invoker>, None)
        };

        {{remove_module_suffix (detect_keyword (to_upper type))}} {
            uri: _uri,
            invoker: _invoker,
            env: _env,
        }
    }

    pub fn default_uri() -> Uri {
        uri!("{{uri}}")
    }

    {{#each methods}}
    pub fn {{detect_keyword (to_lower name)}}(&self, args: &{{ remove_module_suffix (to_upper ../type) }}Args{{to_upper name}}, invoke_options: Option<InvokeOptions>) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, Error> {
        let (_uri, _invoker, _env) = if let Some(invoke_option) = invoke_options {
            let uri = invoke_option.uri.clone().unwrap_or_else(|| self.uri.clone());
            let invoker = invoke_option.client.clone().unwrap_or_else(|| self.invoker.clone());
            let env = invoke_option.env.clone().or_else(|| self.env.clone());
            (uri, invoker, env)
        } else {
            (self.uri.clone(), self.invoker.clone(), self.env.clone())
        };

        let serialized_args = to_vec(&args).unwrap();
        let opt_args = Some(serialized_args.as_slice());
        let result = _invoker.invoke_raw(
            &_uri,
            "{{name}}",
            opt_args,
            _env.as_ref().map(|v| v.as_slice()),
            None
        )?;

        from_slice(result.as_slice()).map_err(Error::MsgpackError)
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
        source: &*SOURCE,
    }
}
