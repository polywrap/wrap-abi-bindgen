lazy_static! {
  static ref NAME: String = "object_type/mod.rs".to_string();
  static ref SOURCE: String = r#"use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    DecodeError,
    EncodeError,
    Read,
    Write,
    JSON,
};
{{#each (property_deps this)}}
use {{_crate}}::{{detect_keyword (to_upper _type)}};
{{/each}}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{serde_keyword (to_lower name)}}pub {{detect_keyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

impl {{detect_keyword (to_upper type)}} {
    pub const URI: &'static str = "{{uri}}";

    pub fn new() -> {{detect_keyword (to_upper type)}} {
        {{detect_keyword (to_upper type)}} {
            {{#each properties}}
            {{detect_keyword (to_lower name)}}: {{to_rust_init (to_graphql_type this)}},
            {{/each}}
        }
    }
}
"#.to_string();
}

use crate::templates::Template;

pub fn load() -> Template {
    Template {
        name: &*NAME,
        source: &*SOURCE
    }
}
