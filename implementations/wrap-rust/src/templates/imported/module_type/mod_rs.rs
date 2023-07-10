lazy_static! {
  static ref NAME: String = "imported/module_type/mod.rs".to_string();
  static ref SOURCE: String = r#"use serde::{Serialize, Deserialize};
use polywrap_msgpack_serde::{
    from_slice,
    to_vec,
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    subinvoke
};
{{#each (property_deps this)}}
use {{_crate}}::{{detect_keyword (to_upper _type)}};
{{/each}}

{{#if (array_has_length methods)}}
{{#each methods}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Args{{to_upper name}} {
    {{#each arguments}}
    {{serde_rename_if_case_mismatch (to_lower name)}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}
{{#if (is_not_last @index ../methods)}}

{{/if}}
{{/each}}
{{/if}}

{{#if isInterface}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    uri: String
}

impl {{detect_keyword (to_upper type)}} {
    pub const INTERFACE_URI: &'static str = "{{uri}}";

    pub fn new(uri: String) -> {{detect_keyword (to_upper type)}} {
        {{detect_keyword (to_upper type)}} { uri }
    }

    {{#each methods}}
    pub fn {{to_lower name}}(&self, args: &Args{{to_upper name}}) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, String> {
        let ref uri = self.uri;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri.as_str(),
            "{{name}}",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
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
    pub fn {{detect_keyword (to_lower name)}}(args: &Args{{to_upper name}}) -> Result<{{#with return}}{{to_rust (to_graphql_type this)}}{{/with}}, String> {
        let uri = {{to_upper ../type}}::URI;
        let args = to_vec(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "{{name}}",
            args,
        )?;
        from_slice(result.as_slice()).map_err(|e| e.to_string())
    }
    {{#if (is_not_last @index ../methods)}}

    {{/if}}
    {{/each}}
}
{{/if}}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
