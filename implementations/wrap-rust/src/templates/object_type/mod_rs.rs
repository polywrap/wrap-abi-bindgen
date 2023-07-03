lazy_static! {
  static ref NAME: String = "object_type/mod.rs".to_string();
  static ref SOURCE: String = r#"use serde::{Serialize, Deserialize};
pub mod serialization;
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
pub use serialization::{
    deserialize_{{to_lower type}},
    read_{{to_lower type}},
    serialize_{{to_lower type}},
    write_{{to_lower type}}
};

{{#each propertyDeps}}
use {{crate}}::{{detect_keyword (to_upper type)}};
{{/each}}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct {{detect_keyword (to_upper type)}} {
    {{#each properties}}
    {{serdeKeyword (to_lower name)}}pub {{detectKeyword (to_lower name)}}: {{to_rust (to_graphql_type this)}},
    {{/each}}
}

impl {{detect_keyword (to_upper type)}} {
    pub fn new() -> {{detect_keyword (to_upper type)}} {
        {{detect_keyword (to_upper type)}} {
            {{#each properties}}
            {{detectKeyword (to_lower name)}}: {{to_rust_init (to_graphql_type this)}},
            {{/each}}
        }
    }

    pub fn to_buffer(args: &{{detect_keyword (to_upper type)}}) -> Result<Vec<u8>, EncodeError> {
        serialize_{{to_lower type}}(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<{{detect_keyword (to_upper type)}}, DecodeError> {
        deserialize_{{to_lower type}}(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &{{detect_keyword (to_upper type)}}, writer: &mut W) -> Result<(), EncodeError> {
        write_{{to_lower type}}(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<{{detect_keyword (to_upper type)}}, DecodeError> {
        read_{{to_lower type}}(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
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
