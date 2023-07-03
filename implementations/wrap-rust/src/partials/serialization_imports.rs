lazy_static! {
  static ref NAME: String = "serialization_imports".to_string();
  static ref SOURCE: String = r#"use std::convert::TryFrom;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
};
use crate::{{detect_keyword (to_upper type)}};
{{#if (array_has_length propertyDeps)}}

{{/if}}{{#each propertyDeps}}
{{#if isEnum}}
use crate::{
    {{detect_keyword (to_upper type)}},
    get_{{to_lower type}}_value,
    sanitize_{{to_lower type}}_value
};
{{else}}
use {{crate}}::{{detect_keyword (to_upper type)}};
{{/if}}
{{/each}}
"#.to_string();
}

use super::Partial;

pub fn load() -> Partial {
    Partial {
        name: &*NAME,
        source: &*SOURCE
    }
}
